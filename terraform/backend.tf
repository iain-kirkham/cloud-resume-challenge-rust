resource "aws_iam_role" "crc_rust_lambda_role" {
  name = "crc_rust_lambda_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Principal = {
        Service = "lambda.amazonaws.com"
      }
    }]
  })
}

resource "aws_iam_role_policy" "lambda_dynamo_permissions" {
  name = "lambda_dynamo_policy"
  role = aws_iam_role.crc_rust_lambda_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Action = [
        "dynamodb:GetItem",
        "dynamodb:UpdateItem"
      ]
      Resource = aws_dynamodb_table.cloud-resume-challenge.arn
    },
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Resource = "*"
      }]
  })
}

resource "aws_lambda_function" "cloud_resume_rust_lambda" {
  function_name = "rust_lambda"
  role          = aws_iam_role.crc_rust_lambda_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2"
  architectures = ["arm64"] # ARM64 specification

  filename         = "./lambda/bootstrap.zip"
  source_code_hash = filebase64sha256("./lambda/bootstrap.zip")

  timeout = 30

  environment {
    variables = {
      TABLE_NAME = aws_dynamodb_table.cloud-resume-challenge.name
    }
  }
}


resource "aws_apigatewayv2_api" "rust_lambda_apigateway" {
  name          = "rust_lambda_api"
  protocol_type = "HTTP"

  cors_configuration {
    allow_credentials = false
    allow_headers     = ["*"]
    allow_methods     = ["POST", "OPTIONS"]
    allow_origins     = [
      "https://iainkirkham.dev",
      "https://www.iainkirkham.dev",
      "http://localhost:4321"
    ]
    expose_headers    = []
    max_age           = 300
  }
}

resource "aws_apigatewayv2_integration" "lambda_integration" {
  api_id           = aws_apigatewayv2_api.rust_lambda_apigateway.id
  integration_type = "AWS_PROXY"
  integration_uri  = aws_lambda_function.cloud_resume_rust_lambda.invoke_arn
}

resource "aws_apigatewayv2_route" "lambda_route" {
  api_id    = aws_apigatewayv2_api.rust_lambda_apigateway.id
  route_key = "POST /{proxy+}"
  target    = "integrations/${aws_apigatewayv2_integration.lambda_integration.id}"
}

resource "aws_apigatewayv2_stage" "lambda_stage" {
  api_id      = aws_apigatewayv2_api.rust_lambda_apigateway.id
  name        = "$default"
  auto_deploy = true
}

resource "aws_lambda_permission" "api_gw_permissions" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.cloud_resume_rust_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.rust_lambda_apigateway.execution_arn}/*/*"
}
