(function() {var implementors = {};
implementors["hyper"] = [{"text":"impl&lt;B&gt; Service&lt;Request&lt;B&gt;&gt; for SendRequest&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Payload + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Service&lt;Name&gt; for GaiResolver","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; Service&lt;Uri&gt; for HttpConnector&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Resolve + Clone + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;R::Future: Send,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;C, B, T&gt; Service&lt;T&gt; for Connect&lt;C, B, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: MakeConnection&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::Connection: Unpin + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::Future: Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::Error: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt; + Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Payload + Unpin + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Data: Unpin,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;C, B&gt; Service&lt;Request&lt;B&gt;&gt; for Client&lt;C, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Connect + Clone + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Payload + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Data: Send,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()