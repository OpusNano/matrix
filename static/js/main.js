(function() {
    'use strict';
    
    // Check for View Transitions API support
    if (!document.startViewTransition) {
        console.log('View Transitions not supported, using fallback');
        return;
    }
    
    // Intercept link clicks for smooth transitions
    document.addEventListener('click', function(e) {
        const link = e.target.closest('a');
        if (!link) return;
        
        const href = link.getAttribute('href');
        if (!href || href.startsWith('#') || href.startsWith('/static/')) return;
        if (e.ctrlKey || e.metaKey || e.shiftKey) return;
        
        e.preventDefault();
        
        document.startViewTransition(function() {
            window.location.href = href;
        });
    });
    
    // Handle back/forward navigation
    window.addEventListener('popstate', function(e) {
        if (!document.startViewTransition) return;
        
        document.startViewTransition(function() {
            // Let the browser handle the navigation
        });
    });
})();
