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
  region = "us-west-2"
}