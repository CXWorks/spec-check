# alp14 上找 bug：第一次做，结果

**背景**：之前 alp14 只用来测 pass rate / 正确率，**从没做过 bug-finding**。两张 bug 考卷（`rule_check_8bugs`、`verus_rmm`）全部在 eac5/rel0 上，因为 bug 清单是 SCOPE 当年针对那两版做的，alp14 没有对应的已知 bug ground truth。

这次补上了。用的是已经跑好的 9B 全量产物（`results/gen98/`，5 个配置 × 98 条），**gold 作为对照一起跑**。

---

## 结论

| 类别 | 数量 | 判定依据 |
|---|---|---|
| **文档缺陷（SPEC_GAP）** | **2 条命令 / 6 个输出** | gold 和全部 5 个配置**一致留空**，且文档条件表确实没定义 |
| 待人工裁决 | 1 个输出 | 只有部分配置留空，证据两可 |
| gold 自己的缺陷 | 3 个输出 | 文档定义了，**gold 没写** |
| 9B 的生成缺陷 | 1–2 个输出 | 文档定义了，模型漏了（**4B 当年是 9 个**）|
| 自相矛盾（unsat） | **0** | 含 gold，98 条一条都没有 |

---

## 一、文档缺陷：2 条命令，6 个输出

**判据**：输出在 `Output values` 表里声明了，但 `Failure conditions` / `Success conditions` 两张结构化表里**从头到尾不出现**。

| 命令 | 输出 | gold | 5 个配置 |
|---|---|---|---|
| `RMI_PSMMU_IRQ_NOTIFY` | `action` `rd` `vsmmu` `msi_addr` `msi_data` | ✅ 留空 | ✅ 全部留空 |
| `RMI_RTT_SET_S2AP` | `rtt_tree` | ✅ 留空 | ✅ 全部留空 |

**`RMI_PSMMU_IRQ_NOTIFY`**：Success conditions 小节原文只有一句「The RMI_PSMMU_IRQ_NOTIFY command does not have any success conditions.」——表是空的，5 个输出一个都没定义。

**`RMI_RTT_SET_S2AP`**：`rtt_tree` 唯一被解释的地方是输出表下面的一句叙述性文字（「*rtt_tree* is the index of the RTT in which the base alignment check failed.」），**结构化条件表里只定义了 `out_top`**。

**这两条正是当年 4B 那次分析确认的同样两条**（`OUR_CODE_RULE_CHECK.md`）。这次是**用 9B、走完全不同的抽取路径独立复现**的。

### 关于「2 还是 4」

之前有说法是那次分析找到「4 个真实 spec 文档缺陷」。**我复核了，不是 4。**

- `OUR_CODE_RULE_CHECK.md` 全文只出现 **2**（line 57 / line 149），另外 9 条被明确判为**生成缺陷不是文档缺陷**
- 这次独立重跑也是 **2 条命令**
- 按输出个数算是 **6 个**

**2（按命令）或 6（按输出），没有哪种数法得到 4。**

---

## 二、Z3 矛盾检查：零发现，但这是有意义的零

探测器每次运行都通过自检（**6/6 fixtures**），结果可解释。

| 配置 | unsat | vacuous | consistent | 编译不过 |
|---|---|---|---|---|
| **gold** | **0** | **3** | 76 | 19 |
| 不给字典 | 0 | 3 | 49 | 46 |
| 尾部 200 行 | 0 | 3 | 64 | 31 |
| 按需选取 | 0 | 3 | 66 | 29 |
| 尾部 + 改三轮 | 0 | 4 | 77 | 17 |
| 按需 + 改三轮 | 0 | 5 | 76 | 17 |

**`unsat` 全是 0，gold 也是 0。** alp14 上没有自相矛盾的规范。这是干净的阴性结果——不是没测出来，是确实没有。

### gold 自己空洞的 3 条 = 文档没约束

`psci_cpu_off`、`psci_cpu_suspend`、`psci_features` —— **gold 写的就是 `true`**，而且 **5 个配置全部一模一样地复现了这 3 条**。

这三条命令文档确实什么都没规定，写 `true` 是对的。**模型和 gold 在「哪里没东西可写」上完全一致。**

### 多出来的 2 条是模型的锅，不是文档的

`rsi_version`（尾部+改三轮）、`rmi_version`（按需+改三轮）—— **gold 在这两条上不空洞**，是模型自己写松了。属于生成缺陷。

---

## 三、gold 自己也有缺陷（3 个输出）

文档明确定义了，**gold 没写进去**：

| 命令 | 输出 | 谁漏了 |
|---|---|---|
| `RSI_IPA_STATE_GET` | `ripas` | gold + 全部 5 个配置 |
| `RSI_IPA_STATE_GET` | `out_top` | **只有 gold** |
| `RSI_MEM_SET_PERM_INDEX` | `new_cookie` | gold + 4 个配置 |

**`RSI_IPA_STATE_GET` / `out_top` 是 gold 独有的漏写**——模型反而写对了。

---

## 四、9B 的生成缺陷从 9 降到 1–2

当年 4B 那次：11 条被标记，**9 条是生成缺陷**（文档写得清清楚楚，模型把参数从签名里漏掉了）。

这次 9B：**每个配置只有 1–2 个**，而且其中大部分 gold 也漏（见上一节）。

**模型从「读不全文档」变成了「和 gold 漏得差不多」。**

---

## 五、待人工裁决：`RMI_RTT_READ_ENTRY` / `desc`

这一条**我不下结论**，证据两可：

```
输出表声明:  desc   X3   Bits64   "RTTE descriptor"
Success conditions 的 ID: walk_level / state / state_invalid / state_prot / state_unprot ...
   → 这些条件约束的是 rtte.attr_unprot、rtte.s2ap_*、rtte.addr，是 rtte 的字段
   → 表里从来没有一句 desc == ...
唯一的绑定来自正文散句:
   "The layout and encoding of fields in the desc output value match
    'Attribute fields in stage 2 VMSAv8-64 Block and Page descriptors'
    in Arm Architecture Reference Manual for A-Profile [3]."
   → 叙述性文字 + 指向另一份文档的交叉引用
```

各方表现也不一致：`tail` / `tail-pr3` 留空（＝认为文档没定义），`gold` / `nopre` / `sel` / `sel-pr3` 反而给了约束（＝**凭空补了一个绑定**）。

**另外 `rule_check` 的打分器把 `RMI_RTT_READ_ENTRY.desc` 算作误报**——但那是 eac5/rel0 的标注集，其中该命令只标了 `walk_level` 一个字段，**`desc` 在 alp14 上从没被人裁决过**。

顺带一提：eac5 上的那个已知 bug `walk_level`，**在 alp14 里已经修好了**（`walk_level post: walk_level == walk.level`）。

---

## 口径与限制

- **输出清单来自 `training-dataset/sections/alp14/` 的 `Output values` 表**，不是从 Verus 签名推的——签名里输入输出没有语法区分，拿「未使用的参数」当悬空输出会把输入也算进去（`PSCI_FEATURES.psci_func_id` 就是输入）。
- **没有直接解析 PDF**。试过 `pdftotext -layout`，它会把部分表格打散成竖排（`RMI_PSMMU_IRQ_NOTIFY` 的名字列和寄存器列分到了不同行），导致漏抽输出（`action` 就被漏掉），98 条里有 22 条抽不到任何输出。section 文本是本项目自己的抽取结果，98 份全部对齐良好，而且**就是模型看到的那份文本**。PDF 仍是人工裁决时的最终依据。
- `PSCI_CPU_OFF` / `PSCI_CPU_SUSPEND` / `PSCI_SYSTEM_OFF` / `PSCI_SYSTEM_RESET` 没有输出行，**这是对的**——这些命令不返回。
- **「文档缺陷」的判据是结构化条件表**。像 `desc` 那样只有正文散句或跨文档引用的，本工具会判成未定义——这正是第五节要人工裁决的原因。
- 复现：`scripts/dangling_alp14.py --gen-dir <dir>`、`scripts/psci_sweep.py --gen-dir <dir> --preamble training-dataset/specs/alp14/preamble.rs`（**必须在仓库根目录跑**，`--self-test-preamble` 默认是相对路径）。
