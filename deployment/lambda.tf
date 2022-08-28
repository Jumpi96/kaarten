data "aws_caller_identity" "current" {}

locals {
  root_dir   = "${path.module}/.."
  account_id = data.aws_caller_identity.current.account_id
  build_args = "--build-arg binary=checkerboard --build-arg log_level=${var.log_level}"
}

resource "aws_ecr_repository" "kaarten_lambda" {
  name = "kaarten-repository"
}

resource "aws_lambda_function" "kaarten" {
  function_name = "kaarten-production"

  image_uri    = "${aws_ecr_repository.kaarten_lambda.repository_url}@${data.aws_ecr_image.lambda_image.id}"
  package_type = "Image"

  timeout     = 10
  memory_size = 128
  role        = aws_iam_role.kaarten.arn
}

resource "null_resource" "lambda_ecr_image_builder" {
  triggers = {
    docker_file     = filesha256("${local.root_dir}/Dockerfile")
    cargo_file      = filesha256("${local.root_dir}/Cargo.toml")
    cargo_lock_file = filesha256("${local.root_dir}/Cargo.lock")
    src_dir         = sha256(join("", [for f in fileset("${local.root_dir}/src", "**") : filesha256("${local.root_dir}/src/${f}")]))
  }

  provisioner "local-exec" {
    working_dir = local.root_dir
    interpreter = ["/bin/bash", "-c"]
    command     = <<-EOT
      aws ecr get-login-password --region ${var.aws_region} | docker login --username AWS --password-stdin ${local.account_id}.dkr.ecr.${var.aws_region}.amazonaws.com
      docker image build -t ${aws_ecr_repository.kaarten_lambda.repository_url}:latest ${local.build_args} .
      docker push ${aws_ecr_repository.kaarten_lambda.repository_url}:latest
    EOT
  }
}

data "aws_ecr_image" "lambda_image" {
  depends_on = [
    null_resource.lambda_ecr_image_builder
  ]

  repository_name = aws_ecr_repository.kaarten_lambda.name
  image_tag       = "latest"
}


resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/${aws_lambda_function.kaarten.function_name}"
  retention_in_days = 30
}

resource "aws_iam_role" "kaarten" {
  name = "kaarten-lambda-iam-role-${var.aws_region}-production"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = ""
        Effect = "Allow"
        Action = "sts:AssumeRole"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "kaarten_role" {
  role       = aws_iam_role.kaarten.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}