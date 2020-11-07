(function() {var implementors = {};
implementors["futures_util"] = [{"text":"impl&lt;A, B&gt; AsyncRead for Either&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: AsyncRead,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: AsyncRead,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;St&gt; AsyncRead for IntoAsyncRead&lt;St&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;St: TryStream&lt;Error = Error&gt; + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;St::Ok: AsRef&lt;[u8]&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsyncRead for AllowStdIo&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Read,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;AsyncRead&gt; AsyncRead for BufReader&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, U&gt; AsyncRead for Chain&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: AsyncRead,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;AsRef&lt;[u8]&gt; + Unpin&gt; AsyncRead for Cursor&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl AsyncRead for Empty","synthetic":false,"types":[]},{"text":"impl AsyncRead for Repeat","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;AsyncRead&gt; AsyncRead for ReadHalf&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;AsyncRead&gt; AsyncRead for Take&lt;R&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()