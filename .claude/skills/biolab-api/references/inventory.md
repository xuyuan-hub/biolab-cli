# Inventory 参考

## 库存状态

```
stored (已入库) → 可用 → checkout (出库) → checkin (入库)
```

## PrimerStockResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 库存 ID |
| primer_name | string | 引物名称 |
| sequence | string | 序列 |
| remaining_quantity | decimal | 剩余量 (nmol) |
| total_quantity | decimal | 总量 (nmol) |
| storage_location_id | UUID | 存储位置 ID |
| location_path | string | 位置路径（如 `冰箱A/盒1`） |
| order_id | UUID | 来源订单 ID |

## PrimerStockDetailResponse

继承 PrimerStockResponse，额外包含：
| 字段 | 说明 |
|------|------|
| transactions | StockTransactionResponse[] | 交易记录列表 |

## StockTransactionResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | |
| type | "checkin" \| "checkout" | 交易类型 |
| quantity | decimal | 数量变化 |
| remaining_after | decimal | 交易后剩余 |
| purpose | string | 用途说明 |
| user_id | UUID | 操作用户 |
| created_at | datetime | 时间 |

## CheckinRequest

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| quantity | decimal | ✓ | 入库量 |
| purpose | string | | 入库原因 |

## CheckoutRequest

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| quantity | decimal | ✓ | 出库量 |
| purpose | string | | 用途 |
| experiment_ref | string | | 实验编号 |

## StorageLocationResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | |
| name | string | 位置名 |
| path | string | 完整路径 |
| children | StorageLocationResponse[] | 子节点 |
| user_id | UUID | 所属用户 |

## 库存列表筛选

`GET /inventory/stocks` 支持：
- `primer_name` — 按名称模糊搜索
- `location_id` — 按位置筛选
- `low_stock` — 仅显示低库存（`true`）

## 库存统计

`GET /inventory/stats` 返回：总库存数、低库存数等摘要。
