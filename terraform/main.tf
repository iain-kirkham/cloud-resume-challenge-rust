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

# The frontend module (S3 + CloudFront + Route53) is no longer deployed here.
# The frontend now runs on Cloudflare Pages, provisioned outside Terraform.
# The module code is kept as a portfolio reference at terraform/legacy/frontend.

