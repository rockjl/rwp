(function ($) {
    "use strict";
    
    /*--
    Menu Stick
    http://www.bootstrapmb.com
    -----------------------------------*/
    var header = $('.sticky-bar');
    var win = $(window);
    win.on('scroll', function() {
        var scroll = win.scrollTop();
        if (scroll < 200) {
            header.removeClass('stick');
        } else {
            header.addClass('stick');
        }
    });
    
    
    /* Slider active */
    $('.slider-active').owlCarousel({
        loop: true,
        nav: true,
        autoplay: false,
        autoplayTimeout: 5000,
        navText: ['<i class="la la-angle-up"></i>', '<i class="la la-angle-down"></i>'],
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        dotsData: true,
        item: 1,
        responsive: {
            0: {
                items: 1
            },
            768: {
                items: 1
            },
            1000: {
                items: 1
            }
        }
    });
    
    /* Slider active 2 */
    $('.slider-active-2').owlCarousel({
        loop: true,
        nav: true,
        autoplay: false,
        autoplayTimeout: 5000,
        navText: ['<i class="la la-angle-left"></i>', '<i class="la la-angle-right"></i>'],
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        item: 1,
        responsive: {
            0: {
                items: 1
            },
            768: {
                items: 1
            },
            1000: {
                items: 1
            }
        }
    });
    
    /* Slider active 3 */
    $('.slider-active-3').owlCarousel({
        loop: true,
        nav: true,
        autoplay: false,
        autoplayTimeout: 5000,
        navText: ['<i class="la la-arrow-left"></i>', '<i class="la la-arrow-right"></i>'],
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        item: 1,
        responsive: {
            0: {
                items: 1
            },
            768: {
                items: 1
            },
            1000: {
                items: 1
            }
        }
    });
    
    /* Slider active 4 */
    $('.slider-active-4').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        item: 1,
        responsive: {
            0: {
                items: 1
            },
            768: {
                items: 1
            },
            1000: {
                items: 1
            }
        }
    });
    
    /* Product slider active */
    $('.product-slider-active').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        item: 4,
        margin: 30,
        responsive: {
            0: {
                items: 1
            },
            576: {
                items: 2
            },
            768: {
                items: 2
            },
            992: {
                items: 3
            },
            1200: {
                items: 4
            }
        }
    });
    
    /* Product slider active 2 */
    $('.product-slider-active-2').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        item: 5,
        responsive: {
            0: {
                items: 1
            },
            576: {
                items: 2
            },
            768: {
                items: 2
            },
            992: {
                items: 3
            },
            1200: {
                items: 5
            }
        }
    });
    
    /* Product slider active 3 */
    $('.product-slider-active-3').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        item: 3,
        margin: 30,
        responsive: {
            0: {
                items: 1
            },
            576: {
                items: 2
            },
            768: {
                items: 2
            },
            992: {
                items: 3
            },
            1200: {
                items: 3
            }
        }
    });
    
    
    /*====== mobile off canvas active ======*/
    function headermobileAside() {
        var navbarTrigger = $('.mobile-aside-button'),
            endTrigger = $('.mobile-aside-close'),
            container = $('.mobile-off-canvas-active'),
            wrapper = $('.main-wrapper');
        
        wrapper.prepend('<div class="body-overlay"></div>');
        
        navbarTrigger.on('click', function(e) {
            e.preventDefault();
            container.addClass('inside');
            wrapper.addClass('overlay-active');
        });
        
        endTrigger.on('click', function() {
            container.removeClass('inside');
            wrapper.removeClass('overlay-active');
        });
        
        $('.body-overlay').on('click', function() {
            container.removeClass('inside');
            wrapper.removeClass('overlay-active');
        });
    };
    headermobileAside();
    
    /*====== clickable mainmenu active ======*/
    function sidebarMenu() {
        var navbarTrigger = $('.clickable-mainmenu-button'),
            endTrigger = $('.clickable-mainmenu-close'),
            container = $('.clickable-mainmenu-active'),
            wrapper = $('.main-wrapper-2');
        
        wrapper.prepend('<div class="body-overlay-2"></div>');
        
        navbarTrigger.on('click', function(e) {
            e.preventDefault();
            container.addClass('inside');
            wrapper.addClass('overlay-active-2');
        });
        
        endTrigger.on('click', function() {
            container.removeClass('inside');
            wrapper.removeClass('overlay-active-2');
        });
        
        $('.body-overlay-2').on('click', function() {
            container.removeClass('inside');
            wrapper.removeClass('overlay-active-2');
        });
    };
    sidebarMenu();
    
    
    /*---------------------
        mobile-menu
    --------------------- */
    var $offCanvasNav = $('.mobile-menu , .sidebar-menu'),
        $offCanvasNavSubMenu = $offCanvasNav.find('.dropdown');
    
    /*Add Toggle Button With Off Canvas Sub Menu*/
    $offCanvasNavSubMenu.parent().prepend('<span class="menu-expand"><i></i></span>');
    
    /*Close Off Canvas Sub Menu*/
    $offCanvasNavSubMenu.slideUp();
    
    /*Category Sub Menu Toggle*/
    $offCanvasNav.on('click', 'li a, li .menu-expand', function(e) {
        var $this = $(this);
        if ( ($this.parent().attr('class').match(/\b(menu-item-has-children|has-children|has-sub-menu)\b/)) && ($this.attr('href') === '#' || $this.hasClass('menu-expand')) ) {
            e.preventDefault();
            if ($this.siblings('ul:visible').length){
                $this.parent('li').removeClass('active');
                $this.siblings('ul').slideUp();
            } else {
                $this.parent('li').addClass('active');
                $this.closest('li').siblings('li').removeClass('active').find('li').removeClass('active');
                $this.closest('li').siblings('li').find('ul:visible').slideUp();
                $this.siblings('ul').slideDown();
            }
        }
    });
    
    
    /*--- language currency active ----*/
    $('.mobile-language-active').on('click', function(e) {
        e.preventDefault();
        $('.lang-dropdown-active').slideToggle(900);
    });
    $('.mobile-currency-active').on('click', function(e) {
        e.preventDefault();
        $('.curr-dropdown-active').slideToggle(900);
    });
    $('.mobile-account-active').on('click', function(e) {
        e.preventDefault();
        $('.account-dropdown-active').slideToggle(900);
    });
    
    
    /*====== Search active ======*/
    function sidebarSearch() {
        var searchTrigger = $('button.search-active'),
            endTriggersearch = $('button.search-close'),
            container = $('.main-search-active');
        searchTrigger.on('click', function() {
            container.addClass('inside');
        });
        endTriggersearch.on('click', function() {
            container.removeClass('inside');
        });
    };
    sidebarSearch();
    
    
    /*--
    Cart active
    -----------------------------------*/
    if ($('.cart-wrap').length) {
        var $body = $('body'),
            $cartWrap = $('.cart-wrap'),
            $cartContent = $cartWrap.find('.shopping-cart-content');
        $cartWrap.on('click', '.cart-active', function(e) {
            e.preventDefault();
            var $this = $(this);
            if (!$this.parent().hasClass('show')) {
                $this.siblings('.shopping-cart-content').addClass('show').parent().addClass('show');
            } else {
                $this.siblings('.shopping-cart-content').removeClass('show').parent().removeClass('show');
            }
        });
        /*Close When Click Close Button*/
        $cartWrap.on('click', '.cart-close', function(e) {
            e.preventDefault();
            var $this = $(this);
            $this.closest('.cart-wrap').removeClass('show').find('.shopping-cart-content').removeClass('show');
        });
        /*Close When Click Outside*/
        $body.on('click', function(e) {
            var $target = e.target;
            if (!$($target).is('.cart-wrap') && !$($target).parents().is('.cart-wrap') && $cartWrap.hasClass('show')) {
                $cartWrap.removeClass('show');
                $cartContent.removeClass('show');
            }
        });
    }
    
    /*--- Quickview slide active ---*/
    $('.quickview-slide-active').owlCarousel({
        loop: true,
        navText: ["<i class='la la-angle-left'></i>", "<i class='la la-angle-right'></i>"],
        margin: 15,
        smartSpeed: 1000,
        nav: true,
        dots: false,
        responsive: {
            0: {
                items: 3,
                autoplay: true,
                smartSpeed: 300
            },
            576: {
                items: 3
            },
            768: {
                items: 3
            },
            1000: {
                items: 3
            }
        }
    })
    
    $('.quickview-slide-active a').on('click', function() {
        $('.quickview-slide-active a').removeClass('active');
    })
    
    /*----------------------------
    	Cart Plus Minus Button
    ------------------------------ */
    var CartPlusMinus = $('.cart-plus-minus');
    CartPlusMinus.prepend('<div class="dec qtybutton">-</div>');
    CartPlusMinus.append('<div class="inc qtybutton">+</div>');
    $(".qtybutton").on("click", function() {
        var $button = $(this);
        var oldValue = $button.parent().find("input").val();
        if ($button.text() === "+") {
            var newVal = parseFloat(oldValue) + 1;
        } else {
            // Don't allow decrementing below zero
            if (oldValue > 0) {
                var newVal = parseFloat(oldValue) - 1;
            } else {
                newVal = 1;
            }
        }
        $button.parent().find("input").val(newVal);
    });
    
    
    /*---------------------
        Video popup
    --------------------- */
    $('.video-popup').magnificPopup({
        type: 'iframe',
        mainClass: 'mfp-fade',
        removalDelay: 160,
        preloader: false, 
        zoom: {
            enabled: true,
        }
    });
    
    /*-----------------------
        Banner slider active
    -----------------------------------*/
    $('.banner-slider-active').slick({
        centerMode: true,
        arrows: false,
        autoplay: false,
        autoplaySpeed: 2000,
        centerPadding: '170px',
        slidesToShow: 3,
        slidesToScroll: 1,
        responsive: [{
                breakpoint: 1500,
                settings: {
                    slidesToShow: 2,
                }
            },
            {
                breakpoint: 1200,
                settings: {
                    slidesToShow: 2,
                }
            },
            {
                breakpoint: 991,
                settings: {
                    slidesToShow: 2,
                    centerPadding: '120px',
                }
            },
            {
                breakpoint: 768,
                settings: {
                    slidesToShow: 1,
                    centerPadding: '100px',
                }
            },
            {
                breakpoint: 575,
                settings: {
                    slidesToShow: 1,
                    centerPadding: '50px',
                }
            }
        ]
    });
    
    
    /*--
    instafeed
    -----------------------------------*/
    // User Changeable Access
    var activeId = $("#instafeed"),
        myTemplate = '<div class="instagram-item"><a href="{{link}}" target="_blank" id="{{id}}"><img src="{{image}}" /></a></div>';
    if (activeId.length) {
        var userID = activeId.attr('data-userid'),
            accessTokenID = activeId.attr('data-accesstoken'),
            userFeed = new Instafeed({
                get: 'user',
                userId: userID,
                accessToken: accessTokenID,
                resolution: 'low_resolution',
                template: myTemplate,
                sortBy: 'least-recent',
                limit: 15,
                links: false
            });
        userFeed.run();
    }
    
    // Instagram feed carousel active
    $(window).on('load', function() {
        /* Testimonial active */
        $('.instagram-carousel').owlCarousel({
            loop: true,
            nav: false,
            autoplay: false,
            autoplayTimeout: 5000,
            item: 5,
            margin: 20,
            responsive: {
                0: {
                    items: 1,
                    autoplay: true,
                },
                576: {
                    items: 3
                },
                768: {
                    items: 4
                },
                1000: {
                    items: 5
                }
            }
        })
        $('.instagram-carousel-2').owlCarousel({
            loop: true,
            nav: false,
            autoplay: false,
            autoplayTimeout: 5000,
            item: 6,
            responsive: {
                0: {
                    items: 1,
                    autoplay: true,
                },
                576: {
                    items: 3
                },
                768: {
                    items: 4
                },
                1000: {
                    items: 6
                }
            }
        })
    })
    
    /*------ Wow Active ----*/
    new WOW().init();
    
    
    /*---------------------
        Countdown
      --------------------- */
    $('[data-countdown]').each(function() {
        var $this = $(this),
            finalDate = $(this).data('countdown');
        $this.countdown(finalDate, function(event) {
            $this.html(event.strftime('<span class="cdown day"> <span>%-D </span><p>Days</p></span> <span class="cdown hour"> <span> %-H</span> <p>Hours</p></span> <span class="cdown minutes"><span>%M</span> <p>Mins</p></span class="cdown second"> <span> <span>%S</span> <p>Secs</p></span>'));
        });
    });
    
    
    /* Brand logo active */
    $('.brand-logo-active').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        item: 4,
        responsive: {
            0: {
                items: 1
            },
            576: {
                items: 2
            },
            768: {
                items: 3
            },
            992: {
                items: 4
            },
            1000: {
                items: 4
            }
        }
    })
    
    /* Brand logo active 2 */
    $('.brand-logo-active-2').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        item: 5,
        margin: 100,
        responsive: {
            0: {
                items: 2,
                margin: 30,
            },
            576: {
                items: 3,
                margin: 50,
            },
            768: {
                items: 3
            },
            992: {
                items: 4
            },
            1000: {
                items: 5
            }
        }
    })
    
    /* Banner slider active 2 */
    $('.banner-slider-active-2').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        item: 3,
        margin: 15,
        responsive: {
            0: {
                items: 2,
            },
            576: {
                items: 3,
            },
            768: {
                items: 3
            },
            992: {
                items: 3
            },
            1000: {
                items: 3
            }
        }
    })
    
    /* Banner slider active 3 */
    $('.banner-slider-active-3').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        item: 3,
        responsive: {
            0: {
                items: 1,
            },
            576: {
                items: 2,
            },
            768: {
                items: 3
            },
            992: {
                items: 3
            },
            1000: {
                items: 3
            }
        }
    })
    
    
    /*--
    Setting active
    -----------------------------------*/
    if ($('.setting-wrap').length) {
        var $body2 = $('body'),
            $elizaDropdown2 = $('.setting-wrap'),
            $elizaDropdownMenu2 = $elizaDropdown2.find('.setting-content');
        $elizaDropdown2.on('click', '.setting-active', function(e) {
            e.preventDefault();
            var $this = $(this);
            if (!$this.parent().hasClass('show')) {
                $this.siblings('.setting-content').addClass('show').slideDown().parent().addClass('show');
            } else {
                $this.siblings('.setting-content').removeClass('show').slideUp().parent().removeClass('show');
            }
        });
        /*Close When Click Outside*/
        $body2.on('click', function(e) {
            var $target = e.target;
            if (!$($target).is('.setting-wrap') && !$($target).parents().is('.setting-wrap') && $elizaDropdown2.hasClass('show')) {
                $elizaDropdown2.removeClass('show');
                $elizaDropdownMenu2.removeClass('show').slideUp();
            }
        });
    }
    
    /* Cart search */
    $(".currency-dropdown-active").on("click", function(e) {
        e.preventDefault();
        $(this).parent().find('.currency-dropdown').slideToggle('medium');
    })
    
    /* hover 3d init for tilt */
    if ($('.tilter').length > 0) {
        $('.tilter').tilt({
            maxTilt: 40,
            perspective: 800,
            easing: "cubic-bezier(.03,.98,.52,.2)",
            scale: 1,
            speed: 800,
            transition: true,
        });
    }
    
    /*-------------------------
      category toggle function
    --------------------------*/
    $('.showcat').on('click', function(e) {
        e.preventDefault();
        $('.hidecat').slideToggle(900);
    });
    
    /* Sidemenu Dropdown */
    function sidemenuDropdown() {
        var $this = $('.mobile-category-menu');
        $this.find('nav .cr-dropdown')
            .find('ul').slideUp();
        $this.find('nav li.cr-dropdown > a, nav li.cr-sub-dropdown > a').on('click', function(e) {
            e.preventDefault();
            $(this).next().slideToggle();
        });
    }
    sidemenuDropdown();
    
    /*--
        Masonry active
    -----------------------------------*/
    $('.grid').imagesLoaded(function() {
        // init Isotope
        var $grid = $('.grid').isotope({
            itemSelector: '.grid-item',
            percentPosition: true,
            layoutMode: 'masonry',
            masonry: {
                // use outer width of grid-sizer for columnWidth
                columnWidth: '.grid-sizer',
            }
        });
    });
    
    /*---------------------
        Price slider
    --------------------- */
    var sliderrange = $('#slider-range');
    var amountprice = $('#amount');
    $(function() {
        sliderrange.slider({
            range: true,
            min: 0,
            max: 1200,
            values: [0, 800],
            slide: function(event, ui) {
                amountprice.val("$" + ui.values[0] + " - $" + ui.values[1]);
            }
        });
        amountprice.val("$" + sliderrange.slider("values", 0) +
            " - $" + sliderrange.slider("values", 1));
    });
    
    
    /*--- checkout toggle function ----*/
    $('.checkout-click1').on('click', function(e) {
        e.preventDefault();
        $('.checkout-login-info').slideToggle(900);
    });
    
    
    /*--- checkout toggle function ----*/
    $('.checkout-click3').on('click', function(e) {
        e.preventDefault();
        $('.checkout-login-info3').slideToggle(1000);
    });
    
    /*-------------------------
    Create an account toggle
    --------------------------*/
    $('.checkout-toggle2').on('click', function() {
        $('.open-toggle2').slideToggle(1000);
    });
    
    $('.checkout-toggle').on('click', function() {
        $('.open-toggle').slideToggle(1000);
    });
    
    /*-------------------------
    checkout one click toggle function
    --------------------------*/
    var checked = $( '.sin-payment input:checked' )
    if(checked){
        $(checked).siblings( '.payment-box' ).slideDown(900);
    };
	 $( '.sin-payment input' ).on('change', function() {
        $( '.payment-box' ).slideUp(900);
        $(this).siblings( '.payment-box' ).slideToggle(900);
    });
    
    /*--------------------------
        ScrollUp
    ---------------------------- */
    $.scrollUp({
        scrollText: '<i class="la la-arrow-up"></i>',
        easingType: 'linear',
        scrollSpeed: 900,
        animation: 'fade'
    });
    
    /*-----------------------
        Product details slider
    --------------------------- */
    $('.product-dec-slider').slick({
        infinite: true,
        slidesToShow: 4,
        slidesToScroll: 1,
        prevArrow: '<span class="product-dec-icon product-dec-prev"><i class="la la-angle-left"></i></span>',
        nextArrow: '<span class="product-dec-icon product-dec-next"><i class="la la-angle-right"></i></span>',
        responsive: [{
                breakpoint: 1199,
                settings: {
                    slidesToShow: 3,
                    slidesToScroll: 1
                }
            },
            {
                breakpoint: 768,
                settings: {
                    slidesToShow: 3,
                    slidesToScroll: 1
                }
            },
            {
                breakpoint: 767,
                settings: {
                    slidesToShow: 4,
                    slidesToScroll: 1
                }
            },
            {
                breakpoint: 575,
                settings: {
                    slidesToShow: 2,
                    slidesToScroll: 1
                }
            }
        ]
    });
    
    
    /*----------------------------
        Product details slider 2
    ------------------------------ */
    $('.product-dec-slider-2').slick({
        infinite: true,
        slidesToShow: 4,
        vertical: true,
        slidesToScroll: 1,
        centerPadding: '60px',
        prevArrow: '<span class="product-dec-icon product-dec-prev"><i class="la la-angle-up"></i></span>',
        nextArrow: '<span class="product-dec-icon product-dec-next"><i class="la la-angle-down"></i></span>',
        responsive: [{
                breakpoint: 1200,
                settings: {
                    slidesToShow: 4,
                    slidesToScroll: 1
                }
            },
            {
                breakpoint: 1199,
                settings: {
                    slidesToShow: 4,
                    slidesToScroll: 1
                }
            },
            {
                breakpoint: 991,
                settings: {
                    slidesToShow: 4,
                    slidesToScroll: 1
                }
            },
            {
                breakpoint: 767,
                settings: {
                    slidesToShow: 4,
                    slidesToScroll: 1
                }
            },
            {
                breakpoint: 575,
                settings: {
                    slidesToShow: 2,
                    slidesToScroll: 1
                }
            }
        ]
    });
    
    
    /*--------------------------
        Product Zoom
	---------------------------- */
    $(".zoompro").elevateZoom({
        gallery: "gallery",
        galleryActiveClass: "active",
        zoomWindowWidth: 300,
        zoomWindowHeight: 100,
        scrollZoom: false,
        zoomType: "inner",
        cursor: "crosshair"
    });
    
    /*---------------------
        Sidebar sticky active
    --------------------- */
    $('.sidebar-active').stickySidebar({
        topSpacing: 80,
        bottomSpacing: 30,
        minWidth: 767,
    });
    
    /* Product details slider */
    $('.box-slider-active').owlCarousel({
        loop: true,
        nav: true,
        autoplay: false,
        autoplayTimeout: 5000,
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        navText: ['<i class="la la-angle-left"></i>', '<i class="la la-angle-right"></i>'],
        item: 3,
        margin: 30,
        responsive: {
            0: {
                items: 1
            },
            576: {
                items: 2
            },
            768: {
                items: 3
            },
            1000: {
                items: 3
            }
        }
    })
    
    
    
    /*--
    Image Popup
    ------------------------*/
    $('.img-popup').magnificPopup({
        type: 'image',
        gallery: {
            enabled: true
        }
    });
    
    
    /* Testimonial active 3 */
    $('.testimonial-active').owlCarousel({
        loop: true,
        nav: false,
        autoplay: false,
        autoplayTimeout: 5000,
        animateOut: 'fadeOut',
        animateIn: 'fadeIn',
        navText: ['<i class="ti-angle-left"></i>', '<i class="ti-angle-right"></i>'],
        item: 2,
        margin: 50,
        responsive: {
            0: {
                items: 1
            },
            768: {
                items: 1
            },
            992: {
                items: 2
            },
            1000: {
                items: 2
            }
        }
    })
    
    
    /*--
    Smooth Scroll
    -----------------------------------*/
    $('.scroll-bottom').on('click', function(e) {
        e.preventDefault();
        var link = this;
        $.smoothScroll({
            offset: 0,
            scrollTarget: link.hash
        });
    });
    
    
    
})(jQuery);

