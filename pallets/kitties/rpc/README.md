kitties的RPC服务：

index -> 遍历

能够查询KittyMarket中的所有的挂单信息（至少包含帐号,kittyIndex），要求返回的信息按照kittyIndex倒序

这个应该是所有人都可以查询，通过KittyCount获取到index的数量，然后反向遍历index，从KittyMarket中进行查询

account -> <kittyIndex, kitty, balance>

<!-- market_info{
    accountInfo：{
        kittyid:{
            Kitty, 
            balance,
        }
    }
} -->