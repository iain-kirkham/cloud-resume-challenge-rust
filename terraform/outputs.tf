# outputs.tf (in the root module)

output "cloudfront_distribution_domain_name" {
  description = "The domain name of the CloudFront distribution"
  value       = module.frontend.cloudfront_distribution_domain_name
}

output "frontend_github_actions_role_arn" {
  description = "The ARN of the frontend IAM role for GitHub Actions"
  value       = module.frontend.frontend_github_actions_role_arn
}

output "frontend_github_actions_role_name" {
  description = "The name of the frontend IAM role for GitHub Actions"
  value       = module.frontend.frontend_github_actions_role_name
}

output "backend_github_actions_role_arn" {
  description = "The ARN of the backend IAM role for GitHub Actions"
  value       = module.backend.github_actions_role_arn
}

output "backend_github_actions_role_name" {
  description = "The name of the backend IAM role for GitHub Actions"
  value       = module.backend.github_actions_role_name
}