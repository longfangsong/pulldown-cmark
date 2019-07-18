// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn table_test_1() {
    let original = r##"Test header
-----------
"##;
    let expected = r##"<h2>Test header</h2>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_2() {
    let original = r##"Test|Table
----|-----
"##;
    let expected = r##"<table><thead><tr><th>Test</th><th>Table</th></tr></thead>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_3() {
    let original = r##"> Test  | Table
> ------|------
> Row 1 | Every
> Row 2 | Day
>
> Paragraph
"##;
    let expected = r##"<blockquote>
<table><thead><tr><th>Test  </th><th> Table</th></tr></thead>
<tr><td>Row 1 </td><td> Every</td></tr>
<tr><td>Row 2 </td><td> Day</td></tr>
</table>
<p>Paragraph</p>
</blockquote>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_4() {
    let original = r##" 1. First entry
 2. Second entry

    Col 1|Col 2
    -|-
    Row 1|Part 2
    Row 2|Part 2
"##;
    let expected = r##"<ol>
<li>
<p>First entry</p>
</li>
<li>
<p>Second entry</p>
<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead>
<tr><td>Row 1</td><td>Part 2</td></tr>
<tr><td>Row 2</td><td>Part 2</td></tr>
</table>
</li>
</ol>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_5() {
    let original = r##"|Col 1|Col 2|
|-----|-----|
|R1C1 |R1C2 |
|R2C1 |R2C2 |
"##;
    let expected = r##"<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead>
<tr><td>R1C1 </td><td>R1C2 </td></tr>
<tr><td>R2C1 </td><td>R2C2 </td></tr>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_6() {
    let original = r##"| Col 1 | Col 2 |
|-------|-------|
|       |       |
|       |       |
"##;
    let expected = r##"<table><thead><tr><th> Col 1 </th><th> Col 2 </th></tr></thead>
<tr><td>       </td><td>       </td></tr>
<tr><td>       </td><td>       </td></tr>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_7() {
    let original = r##"| Col 1 | Col 2 |
|-------|-------|
|   x   |       |
|       |    x  |
"##;
    let expected = r##"<table><thead><tr><th> Col 1 </th><th> Col 2 </th></tr></thead>
<tr><td>   x   </td><td>       </td></tr>
<tr><td>       </td><td>    x  </td></tr>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_8() {
    let original = r##"|Col 1|Col 2|
|-----|-----|
|✓    |✓    |
|✓    |✓    |
"##;
    let expected = r##"<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead>
<tr><td>✓    </td><td>✓    </td></tr>
<tr><td>✓    </td><td>✓    </td></tr>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_9() {
    let original =
        r##"|  Target                       | std |rustc|cargo| notes                      |
|-------------------------------|-----|-----|-----|----------------------------|
| `x86_64-unknown-linux-musl`   |  ✓  |     |     | 64-bit Linux with MUSL     |
| `arm-linux-androideabi`       |  ✓  |     |     | ARM Android                |
| `arm-unknown-linux-gnueabi`   |  ✓  |  ✓  |     | ARM Linux (2.6.18+)        |
| `arm-unknown-linux-gnueabihf` |  ✓  |  ✓  |     | ARM Linux (2.6.18+)        |
| `aarch64-unknown-linux-gnu`   |  ✓  |     |     | ARM64 Linux (2.6.18+)      |
| `mips-unknown-linux-gnu`      |  ✓  |     |     | MIPS Linux (2.6.18+)       |
| `mipsel-unknown-linux-gnu`    |  ✓  |     |     | MIPS (LE) Linux (2.6.18+)  |
"##;
    let expected = r##"<table><thead><tr><th>  Target                       </th><th> std </th><th>rustc</th><th>cargo</th><th> notes                      </th></tr></thead>
<tr><td> <code>x86_64-unknown-linux-musl</code>   </td><td>  ✓  </td><td>     </td><td>     </td><td> 64-bit Linux with MUSL     </td></tr>
<tr><td> <code>arm-linux-androideabi</code>       </td><td>  ✓  </td><td>     </td><td>     </td><td> ARM Android                </td></tr>
<tr><td> <code>arm-unknown-linux-gnueabi</code>   </td><td>  ✓  </td><td>  ✓  </td><td>     </td><td> ARM Linux (2.6.18+)        </td></tr>
<tr><td> <code>arm-unknown-linux-gnueabihf</code> </td><td>  ✓  </td><td>  ✓  </td><td>     </td><td> ARM Linux (2.6.18+)        </td></tr>
<tr><td> <code>aarch64-unknown-linux-gnu</code>   </td><td>  ✓  </td><td>     </td><td>     </td><td> ARM64 Linux (2.6.18+)      </td></tr>
<tr><td> <code>mips-unknown-linux-gnu</code>      </td><td>  ✓  </td><td>     </td><td>     </td><td> MIPS Linux (2.6.18+)       </td></tr>
<tr><td> <code>mipsel-unknown-linux-gnu</code>    </td><td>  ✓  </td><td>     </td><td>     </td><td> MIPS (LE) Linux (2.6.18+)  </td></tr>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_10() {
    let original = r##"|-|-|
|ぃ|い|
"##;
    let expected = r##"<p>|-|-|
|ぃ|い|</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_11() {
    let original = r##"|ぁ|ぃ|
|-|-|
|ぃ|ぃ|
"##;
    let expected = r##"<table><thead><tr><th>ぁ</th><th>ぃ</th></tr></thead>
<tr><td>ぃ</td><td>ぃ</td></tr>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn table_test_12() {
    let original = r##"|Колонка 1|Колонка 2|
|---------|---------|
|Ячейка 1 |Ячейка 2 |
"##;
    let expected = r##"<table><thead><tr><th>Колонка 1</th><th>Колонка 2</th></tr></thead>
<tr><td>Ячейка 1 </td><td>Ячейка 2 </td></tr>
</table>
"##;

    test_markdown_html(original, expected);
}
