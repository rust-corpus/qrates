(function() {var implementors = {};
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T&gt; Pointer for Atomic&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'g, T&gt; Pointer for Shared&lt;'g, T&gt;","synthetic":false,"types":[]}];
implementors["env_logger"] = [{"text":"impl&lt;'a, T:&nbsp;Pointer&gt; Pointer for StyledValue&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["itertools"] = [{"text":"impl&lt;'a, I&gt; Pointer for Format&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Pointer,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()