variable "region" {
  description = "AWS Region to deploy resources"
  type        = string
  default     = "eu-west-2"
}


variable "route53_zone_id" {
  description = "value"
  type        = string
}

variable "acm_certificate_arn" {
  description = "The ARN of the ACM certificate for CloudFront"
  type        = string
}
