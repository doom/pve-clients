# Python client

Example usage:
```python
import pve.nodes

client = MyClient(base_url, token_auth_method(username, realm, token_name, token_secret))

vm = pve.nodes.NodesClient(client).node("proxmox-01").qemu().vmid("123")
print(vm.status().current().get())
```
