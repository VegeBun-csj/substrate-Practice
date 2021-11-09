# kitties的RPC服务：

## 查询KittyMarket中的所有的Kitty信息

```
能够查询KittyMarket中的所有的挂单信息（至少包含帐号,kittyIndex），要求返回的信息按照kittyIndex倒序
```


#### 测试：
> 这里已经自己创建了几个kitty，并调用交易进行了挂单，发起查询请求

```curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "kitty_querykittiymarketinfo"}' http://localhost:9933/```

#### 调用结果：
> 可以看到挂单的kitty的信息

`{"jsonrpc":"2.0","result":[{"info":{"owner":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","price":"0x470de4df820000"},"kittyIndex":4},{"info":{"owner":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","price":"0x2386f26fc10000"},"kittyIndex":2},{"info":{"owner":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","price":"0x2386f26fc10000"},"kittyIndex":0}],"id":1}
`


#### RPC开发的步骤
> 参考substrate代码以及其他项目,代码结构基本一致
1. 相关pallet文件夹与src目录同级，新建一个`rpc`文件夹，其中再新建一个`runtime-api`文件夹.
2. `runtime-api`中定义rpc的trait，这个api是需要装载到`runtime`中的，通过`impl_runtime_apis`引入并实现
3. `rpc`中主要是定义对外暴露的具体的rpc方法，当外部有RPC请求时，会首先请求runtime中的相关rpc api，其中会调用具体的方法（比如这里就是在runtime中调用pallet_kitties中的query_kittiy_market_info方法，这个方法是获取链上的所有market中的kitty数据，并按照kittyIndex倒序显示），然后返回调用结果，对结果进行包裹，作为一个json rpc返回数据，给请求客户端。
> 对于需要特殊处理的字段，比如balance这种u128不可以被Serde序列化，可以在拿到调用结果后，对Balance类型进行处理，再返回
4. pallet文件夹中有一个`rpc.rs`，主要是定义rpc返回值的具体数据类型


#### RPC开发中遇到的问题
1. 自定义的类型在转换为具体类型时，使用`.saturated_into()`
2. 在获取Balance类型的值时，由于Serde库对U128的支持不够，所以采用`NumberOrHex`这个类型来支持u128的序列化(这是个大坑。。。FromStr试过，但是不行，还是官方的NumberOrHex类型靠谱)
3. polkadot.js不支持动态加载自定义的RPC服务，自己构建RPC请求测试即可，比如curl，其他形式的rpc请求
4. 版本问题，当前项目还是使用的8月份的Sub版本
> nightly-2021-08-01-x86_64-unknown-linux-gnu (default)
> rustc 1.56.0-nightly (4e282795d 2021-07-31)
5. 编译打包时出现`getrandom`库的问题，这里得到的方案说是改成edition = 2021，也是一个隐形的大坑，不过也有说法是Cargo.toml里面少std（我遇到了，但是都不是这种解决方式，修其他bug，自然而然就没了）
6. `Cargo.toml`的循环依赖错误，不能出现你依赖我，我依赖你的情况，不是很矛盾？
7. 
