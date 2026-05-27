/**
 * 文件名排序工具模块
 *
 * 目标: 与 Rust 后端 sort_file_items 保持一致的排序行为, 特别是:
 *   1. 数字自然排序 (natural/numeric sorting): "文件2" < "文件10"
 *   2. ASCII 字母大小写不敏感: "a" == "A"
 *   3. 汉字按区域设置排序: "安" < "本" < "新" (an < ben < xin)
 *
 * 排序实现使用浏览器内置的 Intl.Collator API:
 *   - 在所有现代浏览器和 WebView (包括 Tauri WebView2/WKWebView) 中均可用
 *   - 基于 Unicode 排序算法 (UCA), 支持 locale-aware 汉字排序
 *   - 与 Windows 的 CompareStringEx / Unix 的 strcoll 行为类似
 *   - 无需额外依赖, 零体积开销
 *
 * 此算法与 Rust 后端的 compare_filename_natural 函数保持行为一致,
 * 确保前后端排序结果在相同语言环境下完全相同。
 */

/**
 * 区域感知排序器, 使用 zh-CN locale 实现汉字拼音排序
 *
 * 配置说明:
 *   - locale: 'zh-CN' - 中文简体排序规则, 汉字按拼音排序
 *   - sensitivity: 'base' - 大小写不敏感, 'a' 和 'A' 视为相等
 *   - numeric: false - 数字自然排序由手动实现, 确保与后端完全一致
 *
 * 跨平台行为:
 *   - Windows (WebView2/Edge): 使用 Windows NLS API 底层实现, 与 CompareStringEx 一致
 *   - macOS (WKWebView/Safari): 使用 ICU 库, 与 strcoll (zh_CN.UTF-8) 一致
 *   - Linux (WebKitGTK): 使用 ICU 库, 需系统安装中文 locale 支持
 */
const collator = new Intl.Collator("zh-CN", {
  sensitivity: "base",
  numeric: false,
});

/**
 * 判断字符是否为汉字 (CJK Unified Ideographs)
 *
 * 覆盖以下 Unicode 区块:
 *   - CJK Unified Ideographs (U+4E00..U+9FFF)
 *   - CJK Unified Ideographs Extension A (U+3400..U+4DBF)
 *   - CJK Unified Ideographs Extension B..G (U+20000..U+2FA1F)
 */
function isCJKChar(char: string): boolean {
  const code = char.codePointAt(0);
  if (code === undefined) return false;
  return (
    (code >= 0x4e00 && code <= 0x9fff) ||
    (code >= 0x3400 && code <= 0x4dbf) ||
    (code >= 0x20000 && code <= 0x2a6df) ||
    (code >= 0x2a700 && code <= 0x2b73f) ||
    (code >= 0x2b740 && code <= 0x2b81f) ||
    (code >= 0x2b820 && code <= 0x2ceaf) ||
    (code >= 0x2f800 && code <= 0x2fa1f)
  );
}

/**
 * 从字符数组的指定位置提取连续数字, 返回数值和下一个位置索引
 *
 * 修复了原实现中索引不前进的问题 (JS 基本类型按值传递).
 * 现在返回包含 num 和 nextIdx 的对象, 调用方负责更新索引.
 *
 * @param chars - 字符数组
 * @param startIdx - 开始提取的位置
 * @returns { num: 提取的数值, nextIdx: 下一个非数字字符的索引 }
 */
function extractNumber(
  chars: string[],
  startIdx: number
): { num: number; nextIdx: number } {
  let numStr = "";
  let idx = startIdx;
  while (idx < chars.length && /^\d$/.test(chars[idx])) {
    numStr += chars[idx];
    idx++;
  }
  return { num: parseInt(numStr, 10) || 0, nextIdx: idx };
}

/**
 * 自然排序文件名比较函数 (StrCmpLogicalW 风格, 支持区域感知汉字排序)
 *
 * 逐字符交替比较文本段和数字段:
 *   - 数字段按数值比较: "2" < "10"
 *   - ASCII 字母大小写不敏感: "a" == "A"
 *   - 汉字使用 Intl.Collator (zh-CN) 按区域感知规则排序
 *
 * 跨平台排序一致性:
 *   - 在相同 locale 设置下, 前端 Intl.Collator 与后端 CompareStringEx / strcoll 排序结果一致
 *   - 对于 ASCII 字符, 使用手动 case-insensitive 比较以确保与后端完全一致的降级行为
 *
 * @param a - 文件名 A
 * @param b - 文件名 B
 * @returns 负数表示 a < b, 0 表示 a == b, 正数表示 a > b
 */
export function compareFileName(a: string, b: string): number {
  const aChars = Array.from(a);
  const bChars = Array.from(b);
  let aIdx = 0;
  let bIdx = 0;

  while (aIdx < aChars.length && bIdx < bChars.length) {
    const aCh = aChars[aIdx];
    const bCh = bChars[bIdx];

    if (/\d/.test(aCh) && /\d/.test(bCh)) {
      // 两个字符都是数字: 提取完整数字并按数值比较
      const aResult = extractNumber(aChars, aIdx);
      const bResult = extractNumber(bChars, bIdx);
      aIdx = aResult.nextIdx;
      bIdx = bResult.nextIdx;
      if (aResult.num !== bResult.num) {
        return aResult.num - bResult.num;
      }
      // 数字相等, 继续比较后续字符
    } else if (isCJKChar(aCh) && isCJKChar(bCh)) {
      // 两个字符都是汉字: 使用 Intl.Collator (zh-CN) 进行区域感知比较
      const cmp = collator.compare(aCh, bCh);
      if (cmp !== 0) {
        return cmp;
      }
      aIdx++;
      bIdx++;
    } else {
      // 非汉字字符 (ASCII 字母 / 符号 / 其他 Unicode):
      // 手动大小写不敏感比较, 确保与后端 ASCII 比较行为一致
      const aLower = aCh.toLowerCase();
      const bLower = bCh.toLowerCase();
      if (aLower !== bLower) {
        return aLower < bLower ? -1 : 1;
      }
      aIdx++;
      bIdx++;
    }
  }

  return aChars.length - bChars.length;
}