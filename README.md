# Rust Shield
Rust Shield is an implementation of a simple and fast firewall in Rust.
## How to use it
The following are all the commands that you can execute once you start the firewall
```bash
$ ip-address [--range] start-address[:end-address] subnet-mask [-p] [port] deny|allow
```
**Example:**
```bash
$ ip-address 1.1.1.1 255.0.0.0 -p 80 deny
```
Parameters specified in [] are optional. In this particular case, it is possible 
to enter a single address or a pool of addresses by entering the parameter `--range`.

As standard, the firewall allows all packets to pass through. If you want to change 
this setting, you take advantage of the following command:
```bash
$ as-standard allow|any
```

You can also specify a prohibition on passing packets coming from a specific network 
and using a specific port: in the example, we do not allow all packets coming from 1.1.1.1 
to pass through port 80. \
If you wanted to specify a rule only for ports, you can use the following command:
```bash
$ port port-num deny|allow
```
Please note: the firewall is implemented with the same logic as ACLs, thus giving priority 
to the first rule it encounters. \
If you want to view the list of entered rules, the following command is used:
```bash
$ sh rules
```
The rules will have an identifying ID. If you want to delete a rule, you use the following 
command:
```bash
$ rm-rule id-rule
```
