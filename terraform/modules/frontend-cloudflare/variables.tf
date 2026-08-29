variable "zone_id" {
  description = "Cloudflare zone ID for the domain"
  type        = string
}

variable "domain_name" {
  description = "Apex domain name"
  type        = string
  default     = "iainkirkham.dev"
}

variable "ssl_mode" {
  description = "Zone SSL/TLS encryption mode"
  type        = string
  default     = "full"
}

variable "security_level" {
  description = "Zone security level"
  type        = string
  default     = "medium"
}

variable "min_tls_version" {
  description = "Minimum TLS version accepted by the zone"
  type        = string
  default     = "1.0"
}

variable "rate_limit_rules" {
  description = "Rate limiting rules for the zone, expressed as Cloudflare ruleset rules"
  type = list(object({
    ref         = string
    description = string
    expression  = string
    action      = string
    ratelimit = object({
      characteristics     = list(string)
      period              = number
      requests_per_period = number
      mitigation_timeout  = number
    })
  }))
  default = []
}

variable "firewall_rules" {
  description = "Custom firewall/blocking rules for the zone, expressed as Cloudflare ruleset rules"
  type = list(object({
    ref         = string
    description = string
    expression  = string
    action      = string
  }))
  default = []
}
