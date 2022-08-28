terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~>3.27"
    }
  }

  required_version = ">=0.14.9"

  backend "s3" {
    bucket = "kaarten-terraform-state-bucket"
    key    = "state/terraform_state.tfstate"
    region = "us-west-2"
  }
}

provider "aws" {
  version = "~>3.0"
  region = "us-west-2"
}