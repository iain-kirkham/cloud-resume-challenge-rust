output "github_actions_role_arn" {
  description = "The ARN of the IAM role for GitHub Actions"
  value       = aws_iam_role.github_actions_role.arn
}

output "github_actions_role_name" {
  description = "The name of the IAM role for GitHub Actions"
  value       = aws_iam_role.github_actions_role.name
}

output "cloudfront_distribution_domain_name" {
  description = "The domain name of the CloudFront distribution"
  value       = aws_cloudfront_distribution.cloud_resume_challenge_distribution.domain_name
}