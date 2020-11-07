(function() {var implementors = {};
implementors["tokio_executor"] = [];
implementors["tokio_reactor"] = [{"text":"impl Park for Reactor","synthetic":false,"types":[]}];
implementors["tokio_threadpool"] = [{"text":"impl Park for DefaultPark","synthetic":false,"types":[]}];
implementors["tokio_timer"] = [{"text":"impl&lt;T, N&gt; Park for Timer&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Park,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Now,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()