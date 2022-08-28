terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "3.62.0"
    }
  }

  backend "s3" {
    bucket = "botsyn-terraform-state-bucket"
    key    = "state/terraform_state.tfstate"
    region = "us-west-2"
  }
}

provider "aws" {
  region     = var.aws_region
  access_key = var.aws_access_key
  secret_key = var.aws_secret_key
}