(function() {var implementors = {};
implementors["ndarray"] = [{"text":"impl&lt;A, B, S, S2, D, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S2, E&gt;&gt; for <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B, Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = A&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":["ndarray::ArrayBase"]},{"text":"impl&lt;'a, A, B, S, S2, D, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S2, E&gt;&gt; for <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B, Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = A&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":["ndarray::ArrayBase"]},{"text":"impl&lt;'a, A, B, S, S2, D, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S2, E&gt;&gt; for &amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B, Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":["ndarray::ArrayBase"]},{"text":"impl&lt;A, S, D, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B&gt; for <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B, Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = A&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"ndarray/trait.ScalarOperand.html\" title=\"trait ndarray::ScalarOperand\">ScalarOperand</a>,&nbsp;</span>","synthetic":false,"types":["ndarray::ArrayBase"]},{"text":"impl&lt;'a, A, S, D, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B&gt; for &amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B, Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"ndarray/trait.ScalarOperand.html\" title=\"trait ndarray::ScalarOperand\">ScalarOperand</a>,&nbsp;</span>","synthetic":false,"types":["ndarray::ArrayBase"]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i8.html\">i8</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i8.html\">i8</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i8.html\">i8</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i8.html\">i8</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i128.html\">i128</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i128.html\">i128</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i128.html\">i128</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i128.html\">i128</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u128.html\">u128</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u128.html\">u128</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u128.html\">u128</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u128.html\">u128</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.DataOwned.html\" title=\"trait ndarray::DataOwned\">DataOwned</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt; + <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"png/struct.Transformations.html\" title=\"struct png::Transformations\">Transformations</a>&gt; for <a class=\"struct\" href=\"png/struct.Transformations.html\" title=\"struct png::Transformations\">Transformations</a>","synthetic":false,"types":["png::common::Transformations"]}];
implementors["typenum"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>","synthetic":false,"types":["typenum::bit::B0"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>","synthetic":false,"types":["typenum::bit::B1"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>","synthetic":false,"types":["typenum::bit::B0"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>","synthetic":false,"types":["typenum::bit::B1"]},{"text":"impl&lt;Ur:&nbsp;<a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;Ur&gt; for <a class=\"struct\" href=\"typenum/uint/struct.UTerm.html\" title=\"struct typenum::uint::UTerm\">UTerm</a>","synthetic":false,"types":["typenum::uint::UTerm"]},{"text":"impl&lt;Ul:&nbsp;<a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>, Bl:&nbsp;<a class=\"trait\" href=\"typenum/marker_traits/trait.Bit.html\" title=\"trait typenum::marker_traits::Bit\">Bit</a>, Ur:&nbsp;<a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;Ur&gt; for <a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;: PrivateXor&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;PrivateXorOut&lt;<a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;, Ur&gt;: Trim,&nbsp;</span>","synthetic":false,"types":["typenum::uint::UInt"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()