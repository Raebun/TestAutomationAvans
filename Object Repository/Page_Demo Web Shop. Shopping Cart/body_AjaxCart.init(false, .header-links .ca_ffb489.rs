<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_AjaxCart.init(false, .header-links .ca_ffb489</name>
   <tag></tag>
   <elementGuidId>e8782189-2663-457f-9f53-31ff90e71cc1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>body</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>c1020e11-5381-4687-bdcc-74af6a1181c0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    







     





    
    
        
            AjaxCart.init(false, '.header-links .cart-qty', '.header-links .wishlist-qty', '#flyout-cart');
        
        


    
    
        
            
        
    
    
        
    
        
            Register
            Log in
                            
                
                    Shopping cart
                    (2)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $('.header').on('mouseenter', '#topcartlink', function () {
                    $('#flyout-cart').addClass('active');
                });
                $('.header').on('mouseleave', '#topcartlink', function () {
                    $('#flyout-cart').removeClass('active');
                });
                $('.header').on('mouseenter', '#flyout-cart', function () {
                    $('#flyout-cart').addClass('active');
                });
                $('.header').on('mouseleave', '#flyout-cart', function () {
                    $('#flyout-cart').removeClass('active');
                });
            });
        


        
    
        
There are 2 item(s) in your cart.        
            
                    
                            
                                
                                    
                                
                            
                        
                            
                                Blue Jeans
                            
                            Unit price: 1.00
                            Quantity: 1
                        
                    
                    
                            
                                
                                    
                                
                            
                        
                            
                                Smartphone
                            
                            Unit price: 100.00
                            Quantity: 1
                        
                    
            
            Sub-Total: 101.00
            
                    
                            
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == 'Search store') {
                    this.value = '';
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == '') {
                    this.value = 'Search store';
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert('Please enter some search keyword');
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $('#small-searchterms').autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: '/catalog/searchtermautocomplete',
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $('li', '.top-menu').on('mouseenter', function () {
        $('a', $(this)).first().addClass('hover');
        if (!$(this).parent().hasClass('top-menu')) {
            var width = $(this).innerWidth();
            $('.sublist', $(this)).first().css('left', width + 15);
        }
        $('.sublist', $(this)).first().addClass('active');
        $('.top-menu-triangle', $(this)).addClass('active');
    });

    $('li', '.top-menu').on('mouseleave', function () {
        $('a', $(this)).first().removeClass('hover');
        $('.sublist', $(this)).first().removeClass('active');
        $('.top-menu-triangle', $(this)).removeClass('active');
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $('a', $('#mob-menu-button')).toggle(function() {
                $('.mob-top-menu').addClass('show');
            },
            function() {
                $('.mob-top-menu').removeClass('show');
            }
        );

        $(function($) {
            $('.mob-top-menu .expand').click(function() {
                var parent = $(this).parent();
                if (parent.hasClass('active')) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass('active');
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass('active');
                }
            });
        });
    

        
        
        
        
            
            
        
        
            

    
    

    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Smartphone
                                                                                                            
                        
                            Price:
                            100.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            100.00
                        
                    
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            1.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
    
        
            $(function () {
                $(&quot;#CountryId&quot;).change(function () {
                    var selectedItem = $(this).val();
                    var ddlStates = $(&quot;#StateProvinceId&quot;);
                    var estimateProgress = $(&quot;#estimate-shipping-loading-progress&quot;);
                    estimateProgress.show();
                    $.ajax({
                        cache: false,
                        type: &quot;GET&quot;,
                        url: &quot;/country/getstatesbycountryid&quot;,
                 data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                 success: function (data) {
                     ddlStates.html('');
                     $.each(data, function (id, option) {
                         ddlStates.append($('&lt;option>&lt;/option>').val(option.id).html(option.name));
                     });
                     estimateProgress.hide();
                 },
                 error: function (xhr, ajaxOptions, thrownError) {
                     alert('Failed to retrieve states.');
                     estimateProgress.hide();
                 }
             });
                });
            });
        

        
            
                Estimate shipping
            
            Enter your destination to get a shipping estimate
            
                
                    Country:
                    Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D'Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People's Republic of
Kuwait
Kyrgyzstan
Lao People's Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

                    *
                
                
                    State / province:
                    Other (Non US)

                    Wait...
                
                
                    Zip / postal code:
                    
                
                
                    
                
            
        
    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    101.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            0.00
                            
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            101.00
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $('#checkout').click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($('#termsofservice').length > 0) {
                                            //terms of service element exists
                                            if (!$('#termsofservice').is(':checked')) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    


    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2025 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push(['_setAccount', 'UA-6574346-11']);
_gaq.push(['_trackPageview']);

(function() {
    var ga = document.createElement('script'); ga.type = 'text/javascript'; ga.async = true;
    ga.src = ('https:' == document.location.protocol ? 'https://ssl' : 'http://www') + '.google-analytics.com/ga.js';
    var s = document.getElementsByTagName('script')[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]</value>
      <webElementGuid>e6855524-72c7-48b1-bcdc-1bbedd5eb3a7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>6a057d49-4276-4186-9a0d-7d7276271e07</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>c82612b4-ad8e-4085-b8e9-5358a49b8dfb</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    







     





    
    
        
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);
        
        


    
    
        
            
        
    
    
        
    
        
            Register
            Log in
                            
                
                    Shopping cart
                    (2)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
            });
        


        
    
        
There are 2 item(s) in your cart.        
            
                    
                            
                                
                                    
                                
                            
                        
                            
                                Blue Jeans
                            
                            Unit price: 1.00
                            Quantity: 1
                        
                    
                    
                            
                                
                                    
                                
                            
                        
                            
                                Smartphone
                            
                            Unit price: 100.00
                            Quantity: 1
                        
                    
            
            Sub-Total: 101.00
            
                    
                            
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {
            var width = $(this).innerWidth();
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);
        }
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });

    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            },
            function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            }
        );

        $(function($) {
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {
                var parent = $(this).parent();
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    

        
        
        
        
            
            
        
        
            

    
    

    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Smartphone
                                                                                                            
                        
                            Price:
                            100.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            100.00
                        
                    
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            1.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
    
        
            $(function () {
                $(&quot;#CountryId&quot;).change(function () {
                    var selectedItem = $(this).val();
                    var ddlStates = $(&quot;#StateProvinceId&quot;);
                    var estimateProgress = $(&quot;#estimate-shipping-loading-progress&quot;);
                    estimateProgress.show();
                    $.ajax({
                        cache: false,
                        type: &quot;GET&quot;,
                        url: &quot;/country/getstatesbycountryid&quot;,
                 data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                 success: function (data) {
                     ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                     $.each(data, function (id, option) {
                         ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                     });
                     estimateProgress.hide();
                 },
                 error: function (xhr, ajaxOptions, thrownError) {
                     alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                     estimateProgress.hide();
                 }
             });
                });
            });
        

        
            
                Estimate shipping
            
            Enter your destination to get a shipping estimate
            
                
                    Country:
                    Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

                    *
                
                
                    State / province:
                    Other (Non US)

                    Wait...
                
                
                    Zip / postal code:
                    
                
                
                    
                
            
        
    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    101.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            0.00
                            
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            101.00
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $(&quot; , &quot;'&quot; , &quot;#checkout&quot; , &quot;'&quot; , &quot;).click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).length > 0) {
                                            //terms of service element exists
                                            if (!$(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    


    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2025 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);

(function() {
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]&quot;) or . = concat(&quot;
    







     





    
    
        
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);
        
        


    
    
        
            
        
    
    
        
    
        
            Register
            Log in
                            
                
                    Shopping cart
                    (2)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
            });
        


        
    
        
There are 2 item(s) in your cart.        
            
                    
                            
                                
                                    
                                
                            
                        
                            
                                Blue Jeans
                            
                            Unit price: 1.00
                            Quantity: 1
                        
                    
                    
                            
                                
                                    
                                
                            
                        
                            
                                Smartphone
                            
                            Unit price: 100.00
                            Quantity: 1
                        
                    
            
            Sub-Total: 101.00
            
                    
                            
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {
            var width = $(this).innerWidth();
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);
        }
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });

    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            },
            function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            }
        );

        $(function($) {
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {
                var parent = $(this).parent();
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    

        
        
        
        
            
            
        
        
            

    
    

    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Smartphone
                                                                                                            
                        
                            Price:
                            100.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            100.00
                        
                    
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            1.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
    
        
            $(function () {
                $(&quot;#CountryId&quot;).change(function () {
                    var selectedItem = $(this).val();
                    var ddlStates = $(&quot;#StateProvinceId&quot;);
                    var estimateProgress = $(&quot;#estimate-shipping-loading-progress&quot;);
                    estimateProgress.show();
                    $.ajax({
                        cache: false,
                        type: &quot;GET&quot;,
                        url: &quot;/country/getstatesbycountryid&quot;,
                 data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                 success: function (data) {
                     ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                     $.each(data, function (id, option) {
                         ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                     });
                     estimateProgress.hide();
                 },
                 error: function (xhr, ajaxOptions, thrownError) {
                     alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                     estimateProgress.hide();
                 }
             });
                });
            });
        

        
            
                Estimate shipping
            
            Enter your destination to get a shipping estimate
            
                
                    Country:
                    Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

                    *
                
                
                    State / province:
                    Other (Non US)

                    Wait...
                
                
                    Zip / postal code:
                    
                
                
                    
                
            
        
    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    101.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            0.00
                            
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            101.00
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $(&quot; , &quot;'&quot; , &quot;#checkout&quot; , &quot;'&quot; , &quot;).click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).length > 0) {
                                            //terms of service element exists
                                            if (!$(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    


    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2025 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);

(function() {
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]&quot;))]</value>
      <webElementGuid>914c0270-5822-4493-a2dd-d94fcfbd8666</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
