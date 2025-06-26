variable "k8s_host" {
  type = string
}

variable "k8s_certificate" {
  type = string
}

variable "k8s_key" {
  type = string
}

variable "k8s_cluster_ca_certificate" {
  type = string
}

variable "ingress_class" {
  type    = string
  default = "nginx"
}

variable "dns_prefix" {
  type = string
}

variable "nginx_ns" {
  type    = string
  default = "nginx"
}

variable "cert_ns" {
  type    = string
  default = "cert-manager"
}

variable "cert_issuer_staging" {
  type    = string
  default = "letsencrypt-staging-issuer"
}

variable "cert_issuer_prod" {
  type    = string
  default = "letsencrypt-prod-issuer"
}

variable "pgql_database" {
  type    = string
  default = "pgql"
}

variable "image_tag" {
  type = string
}

variable "image_pull_secret" {
  type = string
}

variable "image_pull_username" {
  type = string
}
