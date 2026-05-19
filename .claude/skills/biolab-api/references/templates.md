# 信息模板参考

## 模板字段

| 字段 | 类型 | 说明 | 常用值示例 |
|------|------|------|-----------|
| name | string | 模板名称 | "默认" |
| order_type | string | 订单类型 | "primer_synthesis" / "sequencing" |
| is_default | bool | 设为默认模板 | true |
| company_name | string | 单位名称 | 某某大学/研究所 |
| company_phone | string | 单位电话 | 021-xxxxxxx |
| customer_name | string | 订购人 | 覆盖账户名 |
| customer_phone | string | 手机号 | 138xxxx |
| customer_email | string | 邮箱 | 覆盖账户邮箱 |
| invoice_title | string | 发票抬头 | 同单位名称 |
| principal_investigator | string | 负责人/PI | 导师姓名 |
| payment_method | string | 付款方式 | 月结/现结 |
| recipient_address | string | 收货地址 | 详细地址 |
| weekend_delivery | bool | 双休日发货 | true |
| partial_delivery | bool | 部分先发货 | true |
| confidential | bool | 序列保密 | false |

## 命令

### 创建模板

```bash
cd .claude/skills/biolab-api/scripts; uv run python -c "
from biolab_api import BiolabAPI
api = BiolabAPI()
tpl = api.create_template({
    'name': '默认',
    'order_type': 'primer_synthesis',
    'is_default': True,
    'company_name': '复旦大学',
    'invoice_title': '复旦大学',
    'principal_investigator': '张教授',
    'payment_method': '月结',
    'recipient_address': '上海市杨浦区邯郸路220号',
    'weekend_delivery': True,
    'partial_delivery': True,
    'confidential': False
})
print(f'模板已创建: {tpl[\"id\"]}')
"
```

### 更新模板

```bash
api.update_template('<id>', {'recipient_address': '新地址'})
```

### 删除模板

```bash
api.delete_template('<id>')
```

### 设为默认

```bash
api.set_default_template('<id>')
```
