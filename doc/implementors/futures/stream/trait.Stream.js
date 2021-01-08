(function() {var implementors = {};
implementors["futures"] = [];
implementors["h2"] = [{"text":"impl Stream for PushPromises","synthetic":false,"types":[]},{"text":"impl&lt;T, B&gt; Stream for Connection&lt;T, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: IntoBuf,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Buf: 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Stream for RecvStream","synthetic":false,"types":[]}];
implementors["reqwest"] = [{"text":"impl Stream for Body","synthetic":false,"types":[]},{"text":"impl Stream for Decoder","synthetic":false,"types":[]}];
implementors["tokio_buf"] = [{"text":"impl&lt;T:&nbsp;BufStream&gt; Stream for IntoStream&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio_fs"] = [{"text":"impl Stream for ReadDir","synthetic":false,"types":[]}];
implementors["tokio_io"] = [{"text":"impl&lt;A&gt; Stream for Lines&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: AsyncRead + BufRead,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tokio_signal"] = [{"text":"impl Stream for Signal","synthetic":false,"types":[]}];
implementors["tokio_sync"] = [{"text":"impl&lt;T&gt; Stream for Receiver&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Stream for UnboundedReceiver&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone&gt; Stream for Receiver&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio_tcp"] = [{"text":"impl Stream for Incoming","synthetic":false,"types":[]}];
implementors["tokio_timer"] = [{"text":"impl&lt;T&gt; Stream for DelayQueue&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Stream&gt; Stream for Throttle&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Stream for Timeout&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Stream,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Stream for Interval","synthetic":false,"types":[]}];
implementors["tokio_udp"] = [{"text":"impl&lt;C:&nbsp;Decoder&gt; Stream for UdpFramed&lt;C&gt;","synthetic":false,"types":[]}];
implementors["tokio_uds"] = [{"text":"impl&lt;A, C:&nbsp;Decoder&gt; Stream for UnixDatagramFramed&lt;A, C&gt;","synthetic":false,"types":[]},{"text":"impl Stream for Incoming","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()