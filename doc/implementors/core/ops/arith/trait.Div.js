(function() {var implementors = {};
implementors["nix"] = [{"text":"impl Div&lt;i32&gt; for TimeSpec","synthetic":false,"types":[]},{"text":"impl Div&lt;i32&gt; for TimeVal","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl&lt;'a, 'b&gt; Div&lt;&amp;'b BigNumRef&gt; for &amp;'a BigNumRef","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Div&lt;&amp;'b BigNum&gt; for &amp;'a BigNumRef","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Div&lt;&amp;'b BigNumRef&gt; for &amp;'a BigNum","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Div&lt;&amp;'b BigNum&gt; for &amp;'a BigNum","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl Div&lt;i32&gt; for Duration","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl&lt;I:&nbsp;Integer + NonZero&gt; Div&lt;I&gt; for Z0","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Div&lt;PInt&lt;Ur&gt;&gt; for PInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: Cmp&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;PInt&lt;Ul&gt;: PrivateDivInt&lt;&lt;Ul as Cmp&lt;Ur&gt;&gt;::Output, PInt&lt;Ur&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Div&lt;NInt&lt;Ur&gt;&gt; for PInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: Cmp&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;PInt&lt;Ul&gt;: PrivateDivInt&lt;&lt;Ul as Cmp&lt;Ur&gt;&gt;::Output, NInt&lt;Ur&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Div&lt;PInt&lt;Ur&gt;&gt; for NInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: Cmp&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;NInt&lt;Ul&gt;: PrivateDivInt&lt;&lt;Ul as Cmp&lt;Ur&gt;&gt;::Output, PInt&lt;Ur&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Div&lt;NInt&lt;Ur&gt;&gt; for NInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: Cmp&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;NInt&lt;Ul&gt;: PrivateDivInt&lt;&lt;Ul as Cmp&lt;Ur&gt;&gt;::Output, NInt&lt;Ur&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ur:&nbsp;Unsigned, Br:&nbsp;Bit&gt; Div&lt;UInt&lt;Ur, Br&gt;&gt; for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned, Bl:&nbsp;Bit, Ur:&nbsp;Unsigned, Br:&nbsp;Bit&gt; Div&lt;UInt&lt;Ur, Br&gt;&gt; for UInt&lt;Ul, Bl&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;UInt&lt;Ul, Bl&gt;: Len,<br>&nbsp;&nbsp;&nbsp;&nbsp;Length&lt;UInt&lt;Ul, Bl&gt;&gt;: Sub&lt;B1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;(): PrivateDiv&lt;UInt&lt;Ul, Bl&gt;, UInt&lt;Ur, Br&gt;, U0, U0, Sub1&lt;Length&lt;UInt&lt;Ul, Bl&gt;&gt;&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Rhs&gt; Div&lt;Rhs&gt; for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;V, A, Rhs&gt; Div&lt;Rhs&gt; for TArr&lt;V, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Div&lt;Rhs&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Div&lt;Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()