terraform {
  cloud {
    organization = "iain-kirkham-dev"
    workspaces {
      name = "cloud-resume-challenge-rust"
    }
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region = var.region
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}


module "backend" {
  source = "./modules/backend"
}

# The frontend static site itself (build + deploy) is Cloudflare Pages,
# provisioned outside Terraform, including apex/www DNS which Pages'
# custom domain feature manages automatically. This module covers the
# rest of the zone config: TLS/security settings and rate-limit/WAF rules.
module "frontend_cloudflare" {
  source = "./modules/frontend-cloudflare"

  zone_id     = var.cloudflare_zone_id
  domain_name = var.domain_name

  min_tls_version = var.cloudflare_min_tls_version

  rate_limit_rules = var.cloudflare_rate_limit_rules
  firewall_rules   = var.cloudflare_firewall_rules
}

# The legacy AWS frontend module (S3 + CloudFront + Route53) is no longer deployed here.
# The module code is kept as a portfolio reference at terraform/legacy/frontend.

