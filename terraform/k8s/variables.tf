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
