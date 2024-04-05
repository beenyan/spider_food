# 使用说明

```rs
// 一次發送多少請求，若設太大會被當成惡意流量檔掉
batch_size:
    type: usize
    min: 1
```

```shell
spider_food batch_size=10
```
