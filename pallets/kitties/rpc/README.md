### kitties的RPC服务：

index -> 遍历

能够查询KittyMarket中的所有的挂单信息（至少包含帐号,kittyIndex），要求返回的信息按照kittyIndex倒序

所有人都可以查询，通过KittyCount获取到index的数量，然后反向遍历index，从KittyMarket中进行查询


测试：
```curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "kitty_querykittiymarketinfo"}' http://localhost:9933/```

调用结果：

`{"jsonrpc":"2.0","result":[{"info":{"owner":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","price":"0x470de4df820000"},"kittyIndex":4},{"info":{"owner":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","price":"0x2386f26fc10000"},"kittyIndex":2},{"info":{"owner":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","price":"0x2386f26fc10000"},"kittyIndex":0}],"id":1}
`