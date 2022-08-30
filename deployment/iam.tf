
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

resource "aws_iam_policy" "kaarten_lambda" {
  name        = "iam_policy_kaarten_lambda_function"
  path        = "/"
  description = "IAM policy for the kaarten lambda"
  policy      = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*",
      "Effect": "Allow"
    },
    {
      "Effect": "Allow",
      "Action": "dynamodb:*",
      "Resource": "arn:aws:dynamodb:*:*:table/Collectors"
    }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "kaarten" {
  role       = aws_iam_role.kaarten.name
  policy_arn = aws_iam_policy.kaarten_lambda.arn
}