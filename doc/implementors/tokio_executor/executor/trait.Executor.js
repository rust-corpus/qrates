(function() {var implementors = {};
implementors["tokio_current_thread"] = [{"text":"impl Executor for CurrentThread","synthetic":false,"types":[]},{"text":"impl Executor for TaskExecutor","synthetic":false,"types":[]}];
implementors["tokio_threadpool"] = [{"text":"impl Executor for Sender","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Executor for &amp;'a Sender","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()