variable "region" {
  description = "AWS Region to deploy resources"
  type        = string
  default     = "eu-west-2"
}

variable "bucket_name" {
  description = "value"
  type        = string
  default     = "iain-cloud-site"
}

variable "domain_name" {
  description = "value"
  type        = string
  default     = "iainkirkham.dev"
}

variable "route53_zone_id" {
  description = "value"
  type        = string
}

variable "acm_certificate_arn" {
  description = "The ARN of the ACM certificate for CloudFront"
  type        = string
}

variable "price_class" {
  description = "Price class for CloudFront"
  type        = string
  default     = "PriceClass_100"
}
