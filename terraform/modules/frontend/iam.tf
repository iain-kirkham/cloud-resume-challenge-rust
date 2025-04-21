resource "aws_iam_role" "frontend_github_actions_role" {
  name = "frontend-github-actions-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRoleWithWebIdentity"
        Effect = "Allow"
        Principal = {
          Federated = data.aws_iam_openid_connect_provider.github.arn
        }
        Condition = {
          StringLike = {
            "token.actions.githubusercontent.com:sub" = "repo:iainkirkham/astro-portfolio:*"
          }
        }
      }
    ]
  })
}

resource "aws_iam_role_policy" "frontend_github_actions_policy" {
  name_prefix = "frontend-github-actions-policy-"
  role        = aws_iam_role.frontend_github_actions_role.name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "AllowS3CloudFrontActions"
        Effect = "Allow"
        Action = [
          "s3:PutObject",
          "s3:ListBucket",
          "s3:DeleteObject",
          "cloudfront:CreateInvalidation"
        ]
        Resource = [
          aws_cloudfront_distribution.cloud_resume_challenge_distribution.arn,
          "arn:aws:s3:::${aws_s3_bucket.cloud_resume_bucket.bucket}",
          "arn:aws:s3:::${aws_s3_bucket.cloud_resume_bucket.bucket}/*"
        ]
      }
    ]
  })
}

data "aws_iam_openid_connect_provider" "github" {
  url = "https://token.actions.githubusercontent.com"
}