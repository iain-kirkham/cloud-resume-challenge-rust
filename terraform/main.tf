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
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region = var.region
}


module "backend" {
  source = "./modules/backend"
}

module "frontend" {
  source = "./modules/frontend"
  acm_certificate_arn = var.acm_certificate_arn
  route53_zone_id = var.route53_zone_id
}


