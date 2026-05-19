# Orders 参考

## 状态机

```
pending → ordered → received → stored
 (待下单)  (已下单)  (已收货)   (已入库)
```

- 创建后自动尝试发邮件给供应商
- 发成功 → `ordered`；失败 → 保持 `pending`
- `pending` 订单可点"发送邮件"重试
- PATCH 可手动改状态

## PrimerOrderCreate schema

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| type | `"primer_synthesis"` | ✓ | 固定值 |
| supplier_name | `"sangon"`\|`"biosune"` | ✓ | 供应商 |
| customer_name | string | | 订购人 |
| customer_phone | string | | 手机 |
| customer_email | string | | 邮箱 |
| company_name | string | | 单位 |
| company_phone | string | | 固定电话 |
| invoice_title | string | | 发票抬头 |
| principal_investigator | string | | 负责人 |
| payment_method | string | | 付款方式 |
| recipient_address | string | | 收货地址 |
| notes | string | | 备注 |
| weekend_delivery | bool | | 双休日发货 |
| partial_delivery | bool | | 部分先发货 |
| confidential | bool | | 序列保密 |
| items | PrimerItemCreate[] | ✓ | 至少一条 |

## PrimerItemCreate

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| primer_name | string | ✓ | 引物名称，max 255 |
| sequence | string | ✓ | 序列 5'→3'，max 5000 |
| base_count | int | | 碱基数，默认从 sequence 算 |
| purification_method | string | | HAP/PAGE/HPLC/ULTRAPAGE/IE-HPLC/2X HPLC/PAGE+HPLC/RPC |
| scale_od | decimal | | OD 值 |
| nmoles | decimal | | nmol 值 |
| tube_count | int | | 分装管数 |
| five_modification | string | | 5' 修饰 |
| three_modification | string | | 3' 修饰 |
| double_modification | string | | 双标记修饰 |
| primer_type | string | | qPCR Primer 等 |
| deliverable_form | string | | 干粉/液体 |
| remarks | string | | 备注 |

## 供应商差异

### 生工 (sangon)
- 模板：`sangon-order-template.xlsx`
- Sheet：`引物合成订购表`
- 邮件：`synth@sangon.com`

### 铂尚 (biosune)
- 模板：`biosune_primer_template.xlsx`
- Sheet：`订单表`
- 邮件：`order@biosune.com`

## SequencingOrderCreate schema

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| type | `"sequencing"` | ✓ | 固定值 |
| supplier_name | `"biosune"` | ✓ | 铂尚生物 |
| customer_name | string | | 送样人 |
| customer_phone | string | | 手机 |
| customer_email | string | | 邮箱 |
| invoice_title | string | | 发票抬头 |
| principal_investigator | string | | 负责人 |
| payment_method | string | | 付款方式 |
| recipient_address | string | | 地址 |
| notes | string | | 备注 |
| sample_count | int | | 样品数（自动计算） |
| read_length | string | | 读长 |
| platform | string | | 测序平台 |
| custom_primer1 | string | | 自备引物1名称 |
| custom_primer_sequence1 | string | | 自备引物1序列 |
| custom_primer2 | string | | 自备引物2名称 |
| custom_primer_sequence2 | string | | 自备引物2序列 |
| items | SequencingItemCreate[] | ✓ | 至少一个样品 |
| primer_items | PrimerItemCreate[] | | 可选，需要引物合成时填写 |

## SequencingItemCreate

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | ✓ | 样品名称 |
| type | string | | 菌/质粒/已纯化PCR产物/未纯化PCR产物/其他 |
| seq_vector | string | | 载体名称 |
| vector | string | | 抗生素类型 |
| primers | string | | 引物名称/浓度(pmol/ul) |
| fragment_size | string | | 片段长度或载体大小 |
| reaction_count | int | | 反应个数 |
| universal | bool | | 是否测通 |
| note | string | | 备注 |

## 订单类型标识

| type 值 | 订单类型 | 供应商 |
|---------|----------|--------|
| `primer_synthesis` | 引物合成 | sangon / biosune |
| `sequencing` | 测序 | biosune |

## OrderUpdate (PATCH)

所有字段可选（部分更新），含 `status`。例：
```json
{ "status": "received" }
```
