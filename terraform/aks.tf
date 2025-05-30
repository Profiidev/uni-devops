resource "azurerm_kubernetes_cluster" "k8s" {
  location            = var.location
  name                = var.name
  resource_group_name = var.resource_group_name
  dns_prefix          = var.name
  kubernetes_version  = "1.33"

  identity {
    type = "SystemAssigned"
  }

  default_node_pool {
    name       = "agentpool"
    vm_size    = "Standard_B2s"
    node_count = 1
  }

  network_profile {
    network_plugin    = "kubenet"
    load_balancer_sku = "standard"
  }

  linux_profile {
    admin_username = var.username

    ssh_key {
      key_data = file("${path.module}/authorized_keys")
    }
  }
}

resource "azurerm_public_ip" "public_ip" {
  name                = var.name
  location            = var.location
  resource_group_name = azurerm_kubernetes_cluster.k8s.resource_group_name
  allocation_method   = "Static"
  sku                 = "Standard"
  domain_name_label   = var.name
}
