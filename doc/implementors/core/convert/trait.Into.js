(function() {var implementors = {};
implementors["backtrace"] = [{"text":"impl Into&lt;Vec&lt;BacktraceFrame, Global&gt;&gt; for Backtrace","synthetic":false,"types":[]}];
implementors["corpus_database"] = [{"text":"impl Into&lt;usize&gt; for CrateHash","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for AdtVariantIndex","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for TupleFieldIndex","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for FnParamIndex","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for OperandIndex","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for StatementIndex","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for CallArgIndex","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Module","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Item","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Scope","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for FunctionCall","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Span","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Type","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Field","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Operand","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for BasicBlock","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Statement","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for InternedString","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Package","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for PackageVersion","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Krate","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Edition","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Name","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for RelativeDefId","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for SummaryId","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Abi","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for DefPath","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for Build","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for SpanFileName","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for CrateCfgKey","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for CrateCfgValue","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for TyKind","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for StatementKind","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for BinOp","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for NullOp","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for UnOp","synthetic":false,"types":[]},{"text":"impl Into&lt;usize&gt; for TerminatorKind","synthetic":false,"types":[]},{"text":"impl&lt;K, V0&gt; Into&lt;Vec&lt;(K, V0), Global&gt;&gt; for InterningTable&lt;K, (V0,)&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: InterningTableKey,<br>&nbsp;&nbsp;&nbsp;&nbsp;V0: InterningTableValue,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;K, V0, V1&gt; Into&lt;Vec&lt;(K, V0, V1), Global&gt;&gt; for InterningTable&lt;K, (V0, V1)&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: InterningTableKey,<br>&nbsp;&nbsp;&nbsp;&nbsp;V0: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V1: InterningTableValue,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;K, V0, V1, V2&gt; Into&lt;Vec&lt;(K, V0, V1, V2), Global&gt;&gt; for InterningTable&lt;K, (V0, V1, V2)&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: InterningTableKey,<br>&nbsp;&nbsp;&nbsp;&nbsp;V0: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V1: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V2: InterningTableValue,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;K, V0, V1, V2, V3&gt; Into&lt;Vec&lt;(K, V0, V1, V2, V3), Global&gt;&gt; for InterningTable&lt;K, (V0, V1, V2, V3)&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: InterningTableKey,<br>&nbsp;&nbsp;&nbsp;&nbsp;V0: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V1: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V2: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V3: InterningTableValue,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;K, V0, V1, V2, V3, V4&gt; Into&lt;Vec&lt;(K, V0, V1, V2, V3, V4), Global&gt;&gt; for InterningTable&lt;K, (V0, V1, V2, V3, V4)&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: InterningTableKey,<br>&nbsp;&nbsp;&nbsp;&nbsp;V0: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V1: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V2: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V3: InterningTableValue,<br>&nbsp;&nbsp;&nbsp;&nbsp;V4: InterningTableValue,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; Into&lt;Vec&lt;(K, V), Global&gt;&gt; for InterningTable&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: InterningTableKey,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: InterningTableValue,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Into&lt;Result&lt;R, L&gt;&gt; for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["gimli"] = [{"text":"impl Into&lt;u64&gt; for Pointer","synthetic":false,"types":[]},{"text":"impl&lt;'input, Endian&gt; Into&lt;&amp;'input [u8]&gt; for EndianSlice&lt;'input, Endian&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Endian: Endianity,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["git2"] = [{"text":"impl Into&lt;i32&gt; for TreeWalkResult","synthetic":false,"types":[]},{"text":"impl Into&lt;u32&gt; for TreeWalkMode","synthetic":false,"types":[]}];
implementors["humantime"] = [{"text":"impl Into&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl Into&lt;SystemTime&gt; for Timestamp","synthetic":false,"types":[]}];
implementors["im_rc"] = [{"text":"impl&lt;'a, A&gt; Into&lt;Focus&lt;'a, A&gt;&gt; for FocusMut&lt;'a, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Clone + 'a,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["itertools"] = [{"text":"impl&lt;A, B&gt; Into&lt;Option&lt;Either&lt;A, B&gt;&gt;&gt; for EitherOrBoth&lt;A, B&gt;","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl Into&lt;ucred&gt; for UnixCredentials","synthetic":false,"types":[]}];
implementors["ppv_lite86"] = [{"text":"impl&lt;'a&gt; Into&lt;&amp;'a [u32; 4]&gt; for &amp;'a vec128_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;vec128_storage&gt; for [u32; 4]","synthetic":false,"types":[]},{"text":"impl Into&lt;vec256_storage&gt; for [u64; 4]","synthetic":false,"types":[]},{"text":"impl Into&lt;[u32; 4]&gt; for vec128_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u64; 2]&gt; for vec128_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u128; 1]&gt; for vec128_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u32; 8]&gt; for vec256_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u64; 4]&gt; for vec256_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u128; 2]&gt; for vec256_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u32; 16]&gt; for vec512_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u64; 8]&gt; for vec512_storage","synthetic":false,"types":[]},{"text":"impl Into&lt;[u128; 4]&gt; for vec512_storage","synthetic":false,"types":[]}];
implementors["sized_chunks"] = [{"text":"impl&lt;'a, A:&nbsp;'a, N:&nbsp;ChunkLength&lt;A&gt; + 'a&gt; Into&lt;Slice&lt;'a, A, N&gt;&gt; for SliceMut&lt;'a, A, N&gt;","synthetic":false,"types":[]}];
implementors["unicase"] = [{"text":"impl&lt;'a&gt; Into&lt;&amp;'a str&gt; for UniCase&lt;&amp;'a str&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Into&lt;String&gt; for UniCase&lt;String&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Into&lt;Cow&lt;'a, str&gt;&gt; for UniCase&lt;Cow&lt;'a, str&gt;&gt;","synthetic":false,"types":[]}];
implementors["unicode_bidi"] = [{"text":"impl Into&lt;u8&gt; for Level","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()