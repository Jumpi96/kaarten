resource "aws_apigatewayv2_api" "kaarten_lambda" {
  name          = "kaarten_lambda_gw"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "kaarten_lambda" {
  api_id = aws_apigatewayv2_api.kaarten_lambda.id

  name        = "kaarten_lambda_stage"
  auto_deploy = true

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.kaarten_api_gw.arn

    format = jsonencode({
      requestId               = "$context.requestId"
      sourceIp                = "$context.identity.sourceIp"
      requestTime             = "$context.requestTime"
      protocol                = "$context.protocol"
      httpMethod              = "$context.httpMethod"
      resourcePath            = "$context.resourcePath"
      routeKey                = "$context.routeKey"
      status                  = "$context.status"
      responseLength          = "$context.responseLength"
      integrationErrorMessage = "$context.integrationErrorMessage"
      }
    )
  }

  lifecycle {
    ignore_changes = [deployment_id]
  }
}

resource "aws_apigatewayv2_integration" "kaarten_lambda" {
  api_id = aws_apigatewayv2_api.kaarten_lambda.id

  integration_uri    = aws_lambda_function.kaarten.invoke_arn
  integration_type   = "AWS_PROXY"
  integration_method = "POST"

  lifecycle {
    ignore_changes = [passthrough_behavior]
  }
}

resource "aws_apigatewayv2_route" "kaarten_lambda" {
  api_id = aws_apigatewayv2_api.kaarten_lambda.id

  route_key = "POST /bot"
  target    = "integrations/${aws_apigatewayv2_integration.kaarten_lambda.id}"
}

resource "aws_cloudwatch_log_group" "kaarten_api_gw" {
  name = "/aws/kaarten_api_gw/${aws_apigatewayv2_api.kaarten_lambda.name}"

  retention_in_days = 7
}

resource "aws_lambda_permission" "kaarten_api_gw" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.kaarten.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.kaarten_lambda.execution_arn}/*/*"
}
