# variables.tf | Auth and Application variables

variable "aws_access_key" {
  type        = string
  description = "AWS Access Key"
}

variable "aws_secret_key" {
  type        = string
  description = "AWS Secret Key"
}

variable "aws_region" {
  type        = string
  description = "AWS Region"
}

variable "log_level" {
  type    = string
  default = "info"
}

variable "telegram_token" {
  type = string
}