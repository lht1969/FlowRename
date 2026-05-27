import pinyin from "pinyin";

/**
 * 文件名排序工具模块
 *
 * 目标: 与 Rust 后端 sort_file_items 保持一致的排序行为, 特别是:
 *   1. 数字自然排序 (natural/numeric sorting): "文件2" < "文件10"
 *   2. ASCII 字母大小写不敏感: "a" == "A"
 *   3. 汉字按拼音排序: "安" < "本" < "新" (an < ben < xin)
 *
 * 此算法与 Rust 后端的 compare_filename_natural 函数行为一致,
 * 确保前后端排序结果完全相同。
 */

function isChineseChar(char: string): boolean {
  const code = char.charCodeAt(0);
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

function getPinyin(char: string): string | null {
  if (isChineseChar(char)) {
    const result = pinyin(char, { style: pinyin.STYLE_NORMAL });
    if (result.length > 0 && result[0].length > 0) {
      return result[0][0];
    }
  }
  return null;
}

function extractNumber(chars: string[], idx: number): number {
  let numStr = "";
  while (idx < chars.length && /^\d$/.test(chars[idx])) {
    numStr += chars[idx];
    idx++;
  }
  return parseInt(numStr, 10) || 0;
}

/**
 * 自然排序文件名比较函数 (StrCmpLogicalW 风格, 支持拼音排序)
 *
 * 逐字符交替比较文本段和数字段:
 *   - 数字段按数值比较: "2" < "10"
 *   - ASCII 字母大小写不敏感
 *   - 汉字按拼音比较: "安"(an) < "本"(ben) < "新"(xin)
 *
 * 与 Rust 后端 compare_filename_natural 函数行为一致.
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
      const aNum = extractNumber(aChars, aIdx);
      const bNum = extractNumber(bChars, bIdx);
      if (aNum !== bNum) {
        return aNum - bNum;
      }
    } else {
      const aPinyin = getPinyin(aCh);
      const bPinyin = getPinyin(bCh);

      if (aPinyin !== null && bPinyin !== null) {
        if (aPinyin !== bPinyin) {
          return aPinyin < bPinyin ? -1 : 1;
        }
        aIdx++;
        bIdx++;
      } else if (aPinyin !== null) {
        const aLower = aCh.toLowerCase();
        const bLower = bCh.toLowerCase();
        if (aLower !== bLower) {
          return aLower < bLower ? -1 : 1;
        }
        aIdx++;
        bIdx++;
      } else if (bPinyin !== null) {
        const aLower = aCh.toLowerCase();
        const bLower = bCh.toLowerCase();
        if (aLower !== bLower) {
          return aLower < bLower ? -1 : 1;
        }
        aIdx++;
        bIdx++;
      } else {
        const aLower = aCh.toLowerCase();
        const bLower = bCh.toLowerCase();
        if (aLower !== bLower) {
          return aLower < bLower ? -1 : 1;
        }
        aIdx++;
        bIdx++;
      }
    }
  }

  return aChars.length - bChars.length;
}
