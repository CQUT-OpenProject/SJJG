# 第四章作业（数组和串）

## 一、数组

### 5.2.1 基础题

#### 单项选择题

3. C
4. C
5. D
6. D

#### 填空题

1. `1100`
2. `326`
3. `1208`
4. `42`
5. `i(i+1)/2+j+1`

### 5.2.3 综合习题

#### 2

对应矩阵为：

```text
0  0 10  7  0  0
5  0  0  0 11  0
0  0  0  0  0  0
0  0  0  3  0  0
0  0  0  9  0 11
```

#### 4

1. `k = 2i + j + 1`

2.  
   `i = ⌊k/3⌋`  
   `j = k - 2i - 1`

其中 `A` 的行列下标从 `0` 开始，`B` 的下标从 `1` 开始。

#### 5

1. 上三角矩阵按行优先压缩存储表 `S` 为：

| k | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| S[k] | 0 | 0 | 1 | 0 | 3 | 2 | 0 | 0 | 0 | 0 | 1 | 5 | 1 | 0 | 8 |

当 `1 <= i <= j <= 5` 时：

```text
k = (i-1)(2*5-i+2)/2 + (j-i+1)
```

当 `i > j` 时，`A[i,j] = 0`，不存入压缩表。

2. 稀疏矩阵的三元组表结构可描述为：

```c
#define MAXSIZE 100

typedef struct {
    int row;
    int col;
    int value;
} Triple;

typedef struct {
    Triple data[MAXSIZE];
    int mu;
    int nu;
    int tu;
} TSMatrix;
```

矩阵 `A` 的三元组表为：

| row | col | value |
| --- | --- | --- |
| 1 | 3 | 1 |
| 1 | 5 | 3 |
| 2 | 2 | 2 |
| 3 | 4 | 1 |
| 3 | 5 | 5 |
| 4 | 4 | 1 |
| 5 | 5 | 8 |

`mu = 5`，`nu = 5`，`tu = 7`。

#### 7

下面三元组中的行列下标从 `1` 开始；伪地址从 `0` 开始，按行优先计算。

1. 三元组表示法：

| row | col | value |
| --- | --- | --- |
| 1 | 1 | 15 |
| 1 | 4 | 22 |
| 1 | 6 | -15 |
| 2 | 2 | 13 |
| 2 | 3 | 3 |
| 3 | 4 | -6 |
| 5 | 1 | 91 |

2. 伪地址表示法：

| value | location |
| --- | --- |
| 15 | 0 |
| 22 | 3 |
| -15 | 5 |
| 13 | 7 |
| 3 | 8 |
| -6 | 15 |
| 91 | 24 |

3. 带行指针线性表的单链表表示法：

```text
rhead[1] -> (1, 15) -> (4, 22) -> (6, -15) -> NULL
rhead[2] -> (2, 13) -> (3, 3) -> NULL
rhead[3] -> (4, -6) -> NULL
rhead[4] -> NULL
rhead[5] -> (1, 91) -> NULL
```

4. 十字链表示法：

行链：

```text
r1 -> a11 -> a14 -> a16
r2 -> a22 -> a23
r3 -> a34
r4 -> NULL
r5 -> a51
```

列链：

```text
c1 -> a11 -> a51
c2 -> a22
c3 -> a23
c4 -> a14 -> a34
c5 -> NULL
c6 -> a16
```

结点内容：

| 结点 | row | col | value | right | down |
| --- | --- | --- | --- | --- | --- |
| a11 | 1 | 1 | 15 | a14 | a51 |
| a14 | 1 | 4 | 22 | a16 | a34 |
| a16 | 1 | 6 | -15 | NULL | NULL |
| a22 | 2 | 2 | 13 | a23 | NULL |
| a23 | 2 | 3 | 3 | NULL | NULL |
| a34 | 3 | 4 | -6 | NULL | NULL |
| a51 | 5 | 1 | 91 | NULL | NULL |

## 二、串

### 4.2.1 基础题

#### 单项选择题

1. B
2. B
3. B
4. B
5. D
6. D

#### 填空题

1. `两个串长度相等，且对应位置上的字符都相等`
2.  
   (1) `由一个或多个空格组成的串`  
   (2) `空格字符的个数`
3. `0, 1, 1, 2, 2, 3, 4, 1`
4. `10`
5. `concat(substring(s,1,3), substring(s,7,1))`
6.  
   (1) `1`  
   (2) `3`
7.  
   (1) `7`  
   (2) `4`  
   (3) `8`  
   (4) `1`

### 4.2.2 综合题

#### 5

```text
fun StrReplace(T, P, S):
    // 先查找 P 在 T 中第一次出现的位置
    pos = index(T, P)

    // 没找到时，原串不需要修改
    if pos == 0:
        return T

    // left 保存 P 前面的部分
    left = substring(T, 1, pos - 1)
    // right 保存 P 后面的部分
    right = substring(T, pos + len(P), len(T) - pos - len(P) + 1)

    // 把 left、S、right 重新连接起来
    T = concat(concat(left, S), right)
    return T
```

#### 8

```text
fun StrCmp(S, T):
    // 从两个串的第一个字符开始比较
    i = 1

    while i <= S.length and i <= T.length:
        // 当前字符更大，S 大于 T
        if S.ch[i] > T.ch[i]:
            return 1
        // 当前字符更小，S 小于 T
        if S.ch[i] < T.ch[i]:
            return -1
        // 当前字符相等，继续比较下一个字符
        i = i + 1

    // 前面都相同，再比较两个串的长度
    if S.length == T.length:
        return 0
    if S.length > T.length:
        return 1
    else:
        return -1
```

#### 9

```text
fun concat(T, s1, s2):
    // 新串长度等于两个原串长度之和
    T.length = s1.length + s2.length

    // 两个串都为空时，不需要申请字符空间
    if T.length == 0:
        T.ch = NULL
        return true

    // 在堆区申请保存连接结果的空间
    T.ch = new char[T.length]
    if T.ch == NULL:
        return false

    // 先把 s1 的字符复制到 T 中
    for i = 1 to s1.length:
        T.ch[i] = s1.ch[i]

    // 再把 s2 的字符接到 s1 后面
    for j = 1 to s2.length:
        T.ch[s1.length + j] = s2.ch[j]

    return true
```
