# outputs.tf (in the root module)

output "backend_github_actions_role_arn" {
  description = "The ARN of the backend IAM role for GitHub Actions"
  value       = module.backend.github_actions_role_arn
}

output "backend_github_actions_role_name" {
  description = "The name of the backend IAM role for GitHub Actions"
  value       = module.backend.github_actions_role_name
}