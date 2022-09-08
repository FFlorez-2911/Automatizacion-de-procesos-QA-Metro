<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>DarCLIC</name>
   <tag></tag>
   <elementGuidId>6021787d-d691-4bab-96fb-cc0f400cf73e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='aspnetForm']/div[3]/table/tbody/tr/td/table[4]/tbody/tr/td[3]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value></value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>td</value>
      <webElementGuid>5e30986b-57a9-49fe-9b35-1d63f0d79a57</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>valign</name>
      <type>Main</type>
      <value>top</value>
      <webElementGuid>5badd27d-a900-4215-98ad-49f4ee4a06f0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>colspan</name>
      <type>Main</type>
      <value>2</value>
      <webElementGuid>c743c279-0601-4eac-82a6-45942e32c60a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                                
                                

 
    var comp = 'ctl00_ContentPlaceHolder1_';
    //
    function CheckAll(isOnload) {
        var trk = tch = 0;

        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            var rechazar = document.getElementById(comp + &quot;btnRechazar&quot;);
            var aprobar = document.getElementById(comp + &quot;btnAprobar&quot;);

            if (e.disabled == false) {
                if ((e.name != 'allbox') &amp;&amp; (e.type == 'checkbox')) {
                    if (isOnload != 1) {
                        trk++;
                        e.checked = document.forms[0].allbox.checked;
                        if (e.checked)
                            tch++;
                    }
                    else {
                    }
                }
                else if ((e.name == 'allbox') &amp;&amp; (e.type == 'checkbox')) {
                    if (rechazar.style.visibility === 'hidden') {
                        rechazar.style.visibility = 'visible';
                    } else if (trk == 0) {
                        rechazar.style.visibility = 'hidden';
                    }

                    if (aprobar.style.visibility === 'hidden') {
                        aprobar.style.visibility = 'visible';
                    } else if (trk == 0) {
                        aprobar.style.visibility = 'hidden';
                    }
                } else {
                    if ((tch > 0) &amp;&amp; (rechazar.style.visibility == 'hidden') &amp;&amp; (aprobar.style.visibility == 'hidden')) {
                        rechazar.style.visibility = 'visible';
                        aprobar.style.visibility = 'visible';
                    }
                }
            }
        }
    }

    function CCheckAll() {
        var TB = TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != 'allbox') &amp;&amp; (e.type == 'checkbox')) {
                TB++;
                if (e.checked)
                    TO++;
            }
        }

        if (TO == TB)
            document.forms[0].allbox.checked = true;
        else
            document.forms[0].allbox.checked = false;


        var rechazar = document.getElementById(comp + &quot;btnRechazar&quot;);
        if (rechazar.style.visibility === 'hidden') {
            rechazar.style.visibility = 'visible';
        } else if (TO == 0) {
            rechazar.style.visibility = 'hidden';
        }

        var aprobar = document.getElementById(comp + &quot;btnAprobar&quot;);
        if (aprobar.style.visibility === 'hidden') {
            aprobar.style.visibility = 'visible';
        } else if (TO == 0) {
            aprobar.style.visibility = 'hidden';
        }
    }

    function ValidarRechazo() {
        var TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != 'allbox') &amp;&amp; (e.type == 'checkbox')) {
                if (e.checked)
                    TO++;
            }
        }

        var r = confirm(&quot;¿Está seguro que desea realizar el rechazo de &quot; + TO + &quot; ajustes de carga?&quot;);
        if (r == true) {
            document.getElementById('ctl00_ContentPlaceHolder1_hdfResRec').value = &quot;Si&quot;;
        } else {
            document.getElementById('ctl00_ContentPlaceHolder1_hdfResRec').value = &quot;No&quot;;
        }
    }

    function ValidarAprobacion() {
        var TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != 'allbox') &amp;&amp; (e.type == 'checkbox')) {
                if (e.checked)
                    TO++;
            }
        }

        var r = confirm(&quot;¿Está seguro que desea realizar la aprobación de &quot; + TO + &quot; ajustes de carga?&quot;);
        if (r == true) {
            document.getElementById('ctl00_ContentPlaceHolder1_hdfResApro').value = &quot;Si&quot;;
        } else {
            document.getElementById('ctl00_ContentPlaceHolder1_hdfResApro').value = &quot;No&quot;;
        }
    }

    function AbrirPopupPersonaNaturalTarjeta(nombre, cedula, tarjeta)
    {
        if (nombre == undefined)
           nombre = '';
        if (cedula == undefined)
           cedula = '';
        if (tarjeta == undefined)
           tarjeta = '';
           
        var janela = window.open('../../Usuarios/Registros/frmUsuariosPNTarjetaPopup.aspx?nombre='+nombre+'&amp;cedula='+cedula+'&amp;tarjeta='+tarjeta, 'personaNaturalTarjeta', 'width=600, height=450, scrollbars=yes, status=yes');
    }
    
    function ReceberDadosUsuario(nome, cedula, idPersona, apellidos)
    {
        var Popup = document.getElementById(comp + 'hidPopup');
        if(Popup.value == &quot;Usuario&quot;)
        {               
            var txtIDUsu           = document.getElementById(comp + 'hidIDUsu');
            var txtNombreUsu       = document.getElementById(comp + 'txtNombresUsu');
            var txtApellidosUsu    = document.getElementById(comp + 'txtApellidosUsu');
            var hidNombreUsu       = document.getElementById(comp + 'hidNombresUsu');
            var hidApellidosUsu    = document.getElementById(comp + 'hidApellidosUsu');
            //
            txtIDUsu .value         = idPersona;
            txtNombreUsu .value     = hidNombreUsu.value = nome;
            txtApellidosUsu .value  = hidApellidosUsu.value = apellidos;        
        }
        else
        {
            var txtIDOpe           = document.getElementById(comp + 'hidIDOpe');
            var txtNombreOpe       = document.getElementById(comp + 'txtNombresOpe');
            var txtApellidosOpe    = document.getElementById(comp + 'txtApellidosOpe');
            var hidNombreOpe       = document.getElementById(comp + 'hidNombresOpe');
            var hidApellidosOpe    = document.getElementById(comp + 'hidApellidosOpe');
            //
            txtIDOpe.value         = idPersona;
            txtNombreOpe.value     = hidNombreOpe.value = nome;
            txtApellidosOpe.value  = hidApellidosOpe.value = apellidos;
        }
    }
    
    function Inicializar()
    {
        var txtIDUsu           = document.getElementById(comp + 'hidIDUsu');
        var txtNombreUsu       = document.getElementById(comp + 'txtNombresUsu');
        var txtApellidosUsu    = document.getElementById(comp + 'txtApellidosUsu');
        var hidNombreUsu       = document.getElementById(comp + 'hidNombresUsu');
        var hidApellidosUsu    = document.getElementById(comp + 'hidApellidosUsu');
        //
        txtNombreUsu.value =  hidNombreUsu.value;
        
        txtApellidosUsu.value = hidApellidosUsu.value;
        
        var txtIDOpe           = document.getElementById(comp + 'hidIDOpe');
        var txtNombreOpe       = document.getElementById(comp + 'txtNombresOpe');
        var txtApellidosOpe    = document.getElementById(comp + 'txtApellidosOpe');
        var hidNombreOpe       = document.getElementById(comp + 'hidNombresOpe');
        var hidApellidosOpe    = document.getElementById(comp + 'hidApellidosOpe');
              //
        txtNombreOpe.value =  hidNombreOpe.value;
        
        txtApellidosOpe.value = hidApellidosOpe.value;
    }

    function btnLimparPN_onClick()
    {
        var txtID           = document.getElementById(comp + 'hidID');
        var txtNombre       = document.getElementById(comp + 'txtNombres');
        var txtApellidos    = document.getElementById(comp + 'txtApellidos');
        var hidNombre       = document.getElementById(comp + 'hidNombres');
        var hidApellidos    = document.getElementById(comp + 'hidApellidos');
        //
        txtID.value = txtNombre.value = txtApellidos.value = hidNombre.value = hidApellidos.value = '';
    }
    
   
    function VerificaPessoa(source, arguments)
    {   
        var txtID = document.getElementById(comp + 'hidID');
 
        if (txtID.value == '' )       
        {            
            arguments.IsValid = false;
            return;                       
        }
                        
        arguments.IsValid = true;
     }
    
    function LimparUsu()
    {
            var txtIDUsu           = document.getElementById(comp + 'hidIDUsu');
            var txtNombreUsu       = document.getElementById(comp + 'txtNombresUsu');
            var txtApellidosUsu    = document.getElementById(comp + 'txtApellidosUsu');
            var hidNombreUsu       = document.getElementById(comp + 'hidNombresUsu');
            var hidApellidosUsu    = document.getElementById(comp + 'hidApellidosUsu');
            //
            txtIDUsu .value         = &quot;&quot;;
            txtNombreUsu .value     = &quot;&quot;;
            txtApellidosUsu .value  = &quot;&quot;; 
            hidNombreUsu.value      = &quot;&quot;; 
            hidApellidosUsu.value   = &quot;&quot;; 
    }
    
    function LimparOpe()
    {
            var txtIDOpe           = document.getElementById(comp + 'hidIDOpe');
            var txtNombreOpe       = document.getElementById(comp + 'txtNombresOpe');
            var txtApellidosOpe    = document.getElementById(comp + 'txtApellidosOpe');
            var hidNombreOpe       = document.getElementById(comp + 'hidNombresOpe');
            var hidApellidosOpe    = document.getElementById(comp + 'hidApellidosOpe');
            //
            txtIDOpe.value         = &quot;&quot;;
            txtNombreOpe.value     = &quot;&quot;;
            txtApellidosOpe.value  = &quot;&quot;;
            hidApellidosOpe.value  = &quot;&quot;;
            hidApellidosOpe.value  = &quot;&quot;;
            hidNombreOpe.value = &quot;&quot;;
    }

   
    
    

    
//&lt;![CDATA[
Sys.WebForms.PageRequestManager._initialize('ctl00$ContentPlaceHolder1$scmAjustes', 'aspnetForm', ['tctl00$ContentPlaceHolder1$UpdatePanel4','','tctl00$ContentPlaceHolder1$UpdatePanel5',''], ['ctl00$ContentPlaceHolder1$btnAprobar',''], [], 90, 'ctl00');
//]]>





    
	    
	        Aprobación Ajuste de Carga
	

    
    
        
                
                    Usuario 
                    
            
                     
                    
            
            
                     
                    
            
            
                    
                
                    
                    
            
        
        
            
                Operador 
                
            
                 
                
            
            
                 
                
            
            
                
                
                
            
        
        
            
                Motivo
            
                
	- Seleccione -
	CARGA CANCELADA POR EL SISTEMA
	INGRESO POR PUERTA DE SERVICIO
	DESCUENTO ADICIONAL EN EL TORNIQUETE
	CARGA NO REALIZADA EN EL PUNTO DE VENTA
	REVERSIÓN DE MOVIMIENTO
	SISTEMA ACTUALIZA SALDO ERRADO
	ERROR AL DIGITAR VALOR DE LA CARGA
	TRASLADO DE SALDO
	VENTA CON SALDO DESCUENTA DOBLE
	SISTEMA DUPLICA CARGA
	SISTEMA REALIZA NUEVAMENTE ACTUALIZACIÓN
	INGRESO POR PUERTA DE SERVICIO - TARJETA CON DAÑO ELECTRÓNICO
	TORNIQUETE NO DESCUENTA SALDO
	DEBITO DE SALDO TARJETA AP EXTRAVIADA
	REHABILITACION MANUAL VIAJES X DINERO
	DESTRUCCION POR FALLA DAÑO O DETERIORO
	ERROR DE RECARGA INV
	TARJETA CON DAÑO ELECTRONICO O DEFECTUOSA
	CARGA IRREGULAR
	REHABILITACION MANUAL DINERO SEGUNDAS VIAS
	CADUCIDAD DE TARJETA
	DEBITO SITIADAS AFECT SESION
	DEBITO X ERROR ESCRITURA SIN AFECT SESION
	CADUCIDAD DE SALDO EN TARJETA


        
        
            
                Fecha Inicial
                (DD/MM/AAAA)
            
                
                
                
                
                
                *
            
            
            
                Fecha Fin
                (DD/MM/AAAA)
            
                
                
                
                
                
                *
                
            
            
                
                
        
    
    
    
	
            

	
            
        

    
         
        
            
	
                    
                    
                    
                    
                

                
        
        
    
        
Inicializar();
    


     

                                
                            </value>
      <webElementGuid>a27c26d6-ffbe-49fc-903f-380fc3efe8b4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;aspnetForm&quot;)/div[3]/table[1]/tbody[1]/tr[1]/td[1]/table[@class=&quot;tabBorda&quot;]/tbody[1]/tr[1]/td[3]</value>
      <webElementGuid>25e6af44-548b-429a-8ed6-4c266e0088ad</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='aspnetForm']/div[3]/table/tbody/tr/td/table[4]/tbody/tr/td[3]</value>
      <webElementGuid>fcf4f9be-8af3-47a6-bc0b-b646a72bb582</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Reportes'])[1]/following::td[11]</value>
      <webElementGuid>9b683ed6-af39-4f1c-a87a-9473b40b4a6a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Registros'])[1]/following::td[12]</value>
      <webElementGuid>7e3673bf-5bb7-4789-8e8c-60a9caf67a48</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//table[4]/tbody/tr/td[3]</value>
      <webElementGuid>4d08b1ed-6cb4-47f1-aff4-6d5cd25b8d1a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//td[(text() = concat(&quot;
                                
                                

 
    var comp = &quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_&quot; , &quot;'&quot; , &quot;;
    //
    function CheckAll(isOnload) {
        var trk = tch = 0;

        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            var rechazar = document.getElementById(comp + &quot;btnRechazar&quot;);
            var aprobar = document.getElementById(comp + &quot;btnAprobar&quot;);

            if (e.disabled == false) {
                if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                    if (isOnload != 1) {
                        trk++;
                        e.checked = document.forms[0].allbox.checked;
                        if (e.checked)
                            tch++;
                    }
                    else {
                    }
                }
                else if ((e.name == &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                    if (rechazar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
                        rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                    } else if (trk == 0) {
                        rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
                    }

                    if (aprobar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
                        aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                    } else if (trk == 0) {
                        aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
                    }
                } else {
                    if ((tch > 0) &amp;&amp; (rechazar.style.visibility == &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) &amp;&amp; (aprobar.style.visibility == &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;)) {
                        rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                        aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                    }
                }
            }
        }
    }

    function CCheckAll() {
        var TB = TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                TB++;
                if (e.checked)
                    TO++;
            }
        }

        if (TO == TB)
            document.forms[0].allbox.checked = true;
        else
            document.forms[0].allbox.checked = false;


        var rechazar = document.getElementById(comp + &quot;btnRechazar&quot;);
        if (rechazar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
            rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
        } else if (TO == 0) {
            rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
        }

        var aprobar = document.getElementById(comp + &quot;btnAprobar&quot;);
        if (aprobar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
            aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
        } else if (TO == 0) {
            aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
        }
    }

    function ValidarRechazo() {
        var TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                if (e.checked)
                    TO++;
            }
        }

        var r = confirm(&quot;¿Está seguro que desea realizar el rechazo de &quot; + TO + &quot; ajustes de carga?&quot;);
        if (r == true) {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResRec&quot; , &quot;'&quot; , &quot;).value = &quot;Si&quot;;
        } else {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResRec&quot; , &quot;'&quot; , &quot;).value = &quot;No&quot;;
        }
    }

    function ValidarAprobacion() {
        var TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                if (e.checked)
                    TO++;
            }
        }

        var r = confirm(&quot;¿Está seguro que desea realizar la aprobación de &quot; + TO + &quot; ajustes de carga?&quot;);
        if (r == true) {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResApro&quot; , &quot;'&quot; , &quot;).value = &quot;Si&quot;;
        } else {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResApro&quot; , &quot;'&quot; , &quot;).value = &quot;No&quot;;
        }
    }

    function AbrirPopupPersonaNaturalTarjeta(nombre, cedula, tarjeta)
    {
        if (nombre == undefined)
           nombre = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        if (cedula == undefined)
           cedula = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        if (tarjeta == undefined)
           tarjeta = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
           
        var janela = window.open(&quot; , &quot;'&quot; , &quot;../../Usuarios/Registros/frmUsuariosPNTarjetaPopup.aspx?nombre=&quot; , &quot;'&quot; , &quot;+nombre+&quot; , &quot;'&quot; , &quot;&amp;cedula=&quot; , &quot;'&quot; , &quot;+cedula+&quot; , &quot;'&quot; , &quot;&amp;tarjeta=&quot; , &quot;'&quot; , &quot;+tarjeta, &quot; , &quot;'&quot; , &quot;personaNaturalTarjeta&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;width=600, height=450, scrollbars=yes, status=yes&quot; , &quot;'&quot; , &quot;);
    }
    
    function ReceberDadosUsuario(nome, cedula, idPersona, apellidos)
    {
        var Popup = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidPopup&quot; , &quot;'&quot; , &quot;);
        if(Popup.value == &quot;Usuario&quot;)
        {               
            var txtIDUsu           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDUsu&quot; , &quot;'&quot; , &quot;);
            var txtNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresUsu&quot; , &quot;'&quot; , &quot;);
            var txtApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosUsu&quot; , &quot;'&quot; , &quot;);
            var hidNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresUsu&quot; , &quot;'&quot; , &quot;);
            var hidApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosUsu&quot; , &quot;'&quot; , &quot;);
            //
            txtIDUsu .value         = idPersona;
            txtNombreUsu .value     = hidNombreUsu.value = nome;
            txtApellidosUsu .value  = hidApellidosUsu.value = apellidos;        
        }
        else
        {
            var txtIDOpe           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDOpe&quot; , &quot;'&quot; , &quot;);
            var txtNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresOpe&quot; , &quot;'&quot; , &quot;);
            var txtApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosOpe&quot; , &quot;'&quot; , &quot;);
            var hidNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresOpe&quot; , &quot;'&quot; , &quot;);
            var hidApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosOpe&quot; , &quot;'&quot; , &quot;);
            //
            txtIDOpe.value         = idPersona;
            txtNombreOpe.value     = hidNombreOpe.value = nome;
            txtApellidosOpe.value  = hidApellidosOpe.value = apellidos;
        }
    }
    
    function Inicializar()
    {
        var txtIDUsu           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDUsu&quot; , &quot;'&quot; , &quot;);
        var txtNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresUsu&quot; , &quot;'&quot; , &quot;);
        var txtApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosUsu&quot; , &quot;'&quot; , &quot;);
        var hidNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresUsu&quot; , &quot;'&quot; , &quot;);
        var hidApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosUsu&quot; , &quot;'&quot; , &quot;);
        //
        txtNombreUsu.value =  hidNombreUsu.value;
        
        txtApellidosUsu.value = hidApellidosUsu.value;
        
        var txtIDOpe           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDOpe&quot; , &quot;'&quot; , &quot;);
        var txtNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresOpe&quot; , &quot;'&quot; , &quot;);
        var txtApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosOpe&quot; , &quot;'&quot; , &quot;);
        var hidNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresOpe&quot; , &quot;'&quot; , &quot;);
        var hidApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosOpe&quot; , &quot;'&quot; , &quot;);
              //
        txtNombreOpe.value =  hidNombreOpe.value;
        
        txtApellidosOpe.value = hidApellidosOpe.value;
    }

    function btnLimparPN_onClick()
    {
        var txtID           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidID&quot; , &quot;'&quot; , &quot;);
        var txtNombre       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombres&quot; , &quot;'&quot; , &quot;);
        var txtApellidos    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidos&quot; , &quot;'&quot; , &quot;);
        var hidNombre       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombres&quot; , &quot;'&quot; , &quot;);
        var hidApellidos    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidos&quot; , &quot;'&quot; , &quot;);
        //
        txtID.value = txtNombre.value = txtApellidos.value = hidNombre.value = hidApellidos.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    }
    
   
    function VerificaPessoa(source, arguments)
    {   
        var txtID = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidID&quot; , &quot;'&quot; , &quot;);
 
        if (txtID.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; )       
        {            
            arguments.IsValid = false;
            return;                       
        }
                        
        arguments.IsValid = true;
     }
    
    function LimparUsu()
    {
            var txtIDUsu           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDUsu&quot; , &quot;'&quot; , &quot;);
            var txtNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresUsu&quot; , &quot;'&quot; , &quot;);
            var txtApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosUsu&quot; , &quot;'&quot; , &quot;);
            var hidNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresUsu&quot; , &quot;'&quot; , &quot;);
            var hidApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosUsu&quot; , &quot;'&quot; , &quot;);
            //
            txtIDUsu .value         = &quot;&quot;;
            txtNombreUsu .value     = &quot;&quot;;
            txtApellidosUsu .value  = &quot;&quot;; 
            hidNombreUsu.value      = &quot;&quot;; 
            hidApellidosUsu.value   = &quot;&quot;; 
    }
    
    function LimparOpe()
    {
            var txtIDOpe           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDOpe&quot; , &quot;'&quot; , &quot;);
            var txtNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresOpe&quot; , &quot;'&quot; , &quot;);
            var txtApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosOpe&quot; , &quot;'&quot; , &quot;);
            var hidNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresOpe&quot; , &quot;'&quot; , &quot;);
            var hidApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosOpe&quot; , &quot;'&quot; , &quot;);
            //
            txtIDOpe.value         = &quot;&quot;;
            txtNombreOpe.value     = &quot;&quot;;
            txtApellidosOpe.value  = &quot;&quot;;
            hidApellidosOpe.value  = &quot;&quot;;
            hidApellidosOpe.value  = &quot;&quot;;
            hidNombreOpe.value = &quot;&quot;;
    }

   
    
    

    
//&lt;![CDATA[
Sys.WebForms.PageRequestManager._initialize(&quot; , &quot;'&quot; , &quot;ctl00$ContentPlaceHolder1$scmAjustes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;aspnetForm&quot; , &quot;'&quot; , &quot;, [&quot; , &quot;'&quot; , &quot;tctl00$ContentPlaceHolder1$UpdatePanel4&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;tctl00$ContentPlaceHolder1$UpdatePanel5&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;], [&quot; , &quot;'&quot; , &quot;ctl00$ContentPlaceHolder1$btnAprobar&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;], [], 90, &quot; , &quot;'&quot; , &quot;ctl00&quot; , &quot;'&quot; , &quot;);
//]]>





    
	    
	        Aprobación Ajuste de Carga
	

    
    
        
                
                    Usuario 
                    
            
                     
                    
            
            
                     
                    
            
            
                    
                
                    
                    
            
        
        
            
                Operador 
                
            
                 
                
            
            
                 
                
            
            
                
                
                
            
        
        
            
                Motivo
            
                
	- Seleccione -
	CARGA CANCELADA POR EL SISTEMA
	INGRESO POR PUERTA DE SERVICIO
	DESCUENTO ADICIONAL EN EL TORNIQUETE
	CARGA NO REALIZADA EN EL PUNTO DE VENTA
	REVERSIÓN DE MOVIMIENTO
	SISTEMA ACTUALIZA SALDO ERRADO
	ERROR AL DIGITAR VALOR DE LA CARGA
	TRASLADO DE SALDO
	VENTA CON SALDO DESCUENTA DOBLE
	SISTEMA DUPLICA CARGA
	SISTEMA REALIZA NUEVAMENTE ACTUALIZACIÓN
	INGRESO POR PUERTA DE SERVICIO - TARJETA CON DAÑO ELECTRÓNICO
	TORNIQUETE NO DESCUENTA SALDO
	DEBITO DE SALDO TARJETA AP EXTRAVIADA
	REHABILITACION MANUAL VIAJES X DINERO
	DESTRUCCION POR FALLA DAÑO O DETERIORO
	ERROR DE RECARGA INV
	TARJETA CON DAÑO ELECTRONICO O DEFECTUOSA
	CARGA IRREGULAR
	REHABILITACION MANUAL DINERO SEGUNDAS VIAS
	CADUCIDAD DE TARJETA
	DEBITO SITIADAS AFECT SESION
	DEBITO X ERROR ESCRITURA SIN AFECT SESION
	CADUCIDAD DE SALDO EN TARJETA


        
        
            
                Fecha Inicial
                (DD/MM/AAAA)
            
                
                
                
                
                
                *
            
            
            
                Fecha Fin
                (DD/MM/AAAA)
            
                
                
                
                
                
                *
                
            
            
                
                
        
    
    
    
	
            

	
            
        

    
         
        
            
	
                    
                    
                    
                    
                

                
        
        
    
        
Inicializar();
    


     

                                
                            &quot;) or . = concat(&quot;
                                
                                

 
    var comp = &quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_&quot; , &quot;'&quot; , &quot;;
    //
    function CheckAll(isOnload) {
        var trk = tch = 0;

        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            var rechazar = document.getElementById(comp + &quot;btnRechazar&quot;);
            var aprobar = document.getElementById(comp + &quot;btnAprobar&quot;);

            if (e.disabled == false) {
                if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                    if (isOnload != 1) {
                        trk++;
                        e.checked = document.forms[0].allbox.checked;
                        if (e.checked)
                            tch++;
                    }
                    else {
                    }
                }
                else if ((e.name == &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                    if (rechazar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
                        rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                    } else if (trk == 0) {
                        rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
                    }

                    if (aprobar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
                        aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                    } else if (trk == 0) {
                        aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
                    }
                } else {
                    if ((tch > 0) &amp;&amp; (rechazar.style.visibility == &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) &amp;&amp; (aprobar.style.visibility == &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;)) {
                        rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                        aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
                    }
                }
            }
        }
    }

    function CCheckAll() {
        var TB = TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                TB++;
                if (e.checked)
                    TO++;
            }
        }

        if (TO == TB)
            document.forms[0].allbox.checked = true;
        else
            document.forms[0].allbox.checked = false;


        var rechazar = document.getElementById(comp + &quot;btnRechazar&quot;);
        if (rechazar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
            rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
        } else if (TO == 0) {
            rechazar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
        }

        var aprobar = document.getElementById(comp + &quot;btnAprobar&quot;);
        if (aprobar.style.visibility === &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
            aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot;;
        } else if (TO == 0) {
            aprobar.style.visibility = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
        }
    }

    function ValidarRechazo() {
        var TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                if (e.checked)
                    TO++;
            }
        }

        var r = confirm(&quot;¿Está seguro que desea realizar el rechazo de &quot; + TO + &quot; ajustes de carga?&quot;);
        if (r == true) {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResRec&quot; , &quot;'&quot; , &quot;).value = &quot;Si&quot;;
        } else {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResRec&quot; , &quot;'&quot; , &quot;).value = &quot;No&quot;;
        }
    }

    function ValidarAprobacion() {
        var TO = 0;
        for (var i = 0; i &lt; document.forms[0].elements.length; i++) {
            var e = document.forms[0].elements[i];
            if ((e.name != &quot; , &quot;'&quot; , &quot;allbox&quot; , &quot;'&quot; , &quot;) &amp;&amp; (e.type == &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot;)) {
                if (e.checked)
                    TO++;
            }
        }

        var r = confirm(&quot;¿Está seguro que desea realizar la aprobación de &quot; + TO + &quot; ajustes de carga?&quot;);
        if (r == true) {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResApro&quot; , &quot;'&quot; , &quot;).value = &quot;Si&quot;;
        } else {
            document.getElementById(&quot; , &quot;'&quot; , &quot;ctl00_ContentPlaceHolder1_hdfResApro&quot; , &quot;'&quot; , &quot;).value = &quot;No&quot;;
        }
    }

    function AbrirPopupPersonaNaturalTarjeta(nombre, cedula, tarjeta)
    {
        if (nombre == undefined)
           nombre = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        if (cedula == undefined)
           cedula = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        if (tarjeta == undefined)
           tarjeta = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
           
        var janela = window.open(&quot; , &quot;'&quot; , &quot;../../Usuarios/Registros/frmUsuariosPNTarjetaPopup.aspx?nombre=&quot; , &quot;'&quot; , &quot;+nombre+&quot; , &quot;'&quot; , &quot;&amp;cedula=&quot; , &quot;'&quot; , &quot;+cedula+&quot; , &quot;'&quot; , &quot;&amp;tarjeta=&quot; , &quot;'&quot; , &quot;+tarjeta, &quot; , &quot;'&quot; , &quot;personaNaturalTarjeta&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;width=600, height=450, scrollbars=yes, status=yes&quot; , &quot;'&quot; , &quot;);
    }
    
    function ReceberDadosUsuario(nome, cedula, idPersona, apellidos)
    {
        var Popup = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidPopup&quot; , &quot;'&quot; , &quot;);
        if(Popup.value == &quot;Usuario&quot;)
        {               
            var txtIDUsu           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDUsu&quot; , &quot;'&quot; , &quot;);
            var txtNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresUsu&quot; , &quot;'&quot; , &quot;);
            var txtApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosUsu&quot; , &quot;'&quot; , &quot;);
            var hidNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresUsu&quot; , &quot;'&quot; , &quot;);
            var hidApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosUsu&quot; , &quot;'&quot; , &quot;);
            //
            txtIDUsu .value         = idPersona;
            txtNombreUsu .value     = hidNombreUsu.value = nome;
            txtApellidosUsu .value  = hidApellidosUsu.value = apellidos;        
        }
        else
        {
            var txtIDOpe           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDOpe&quot; , &quot;'&quot; , &quot;);
            var txtNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresOpe&quot; , &quot;'&quot; , &quot;);
            var txtApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosOpe&quot; , &quot;'&quot; , &quot;);
            var hidNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresOpe&quot; , &quot;'&quot; , &quot;);
            var hidApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosOpe&quot; , &quot;'&quot; , &quot;);
            //
            txtIDOpe.value         = idPersona;
            txtNombreOpe.value     = hidNombreOpe.value = nome;
            txtApellidosOpe.value  = hidApellidosOpe.value = apellidos;
        }
    }
    
    function Inicializar()
    {
        var txtIDUsu           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDUsu&quot; , &quot;'&quot; , &quot;);
        var txtNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresUsu&quot; , &quot;'&quot; , &quot;);
        var txtApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosUsu&quot; , &quot;'&quot; , &quot;);
        var hidNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresUsu&quot; , &quot;'&quot; , &quot;);
        var hidApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosUsu&quot; , &quot;'&quot; , &quot;);
        //
        txtNombreUsu.value =  hidNombreUsu.value;
        
        txtApellidosUsu.value = hidApellidosUsu.value;
        
        var txtIDOpe           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDOpe&quot; , &quot;'&quot; , &quot;);
        var txtNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresOpe&quot; , &quot;'&quot; , &quot;);
        var txtApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosOpe&quot; , &quot;'&quot; , &quot;);
        var hidNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresOpe&quot; , &quot;'&quot; , &quot;);
        var hidApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosOpe&quot; , &quot;'&quot; , &quot;);
              //
        txtNombreOpe.value =  hidNombreOpe.value;
        
        txtApellidosOpe.value = hidApellidosOpe.value;
    }

    function btnLimparPN_onClick()
    {
        var txtID           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidID&quot; , &quot;'&quot; , &quot;);
        var txtNombre       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombres&quot; , &quot;'&quot; , &quot;);
        var txtApellidos    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidos&quot; , &quot;'&quot; , &quot;);
        var hidNombre       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombres&quot; , &quot;'&quot; , &quot;);
        var hidApellidos    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidos&quot; , &quot;'&quot; , &quot;);
        //
        txtID.value = txtNombre.value = txtApellidos.value = hidNombre.value = hidApellidos.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    }
    
   
    function VerificaPessoa(source, arguments)
    {   
        var txtID = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidID&quot; , &quot;'&quot; , &quot;);
 
        if (txtID.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; )       
        {            
            arguments.IsValid = false;
            return;                       
        }
                        
        arguments.IsValid = true;
     }
    
    function LimparUsu()
    {
            var txtIDUsu           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDUsu&quot; , &quot;'&quot; , &quot;);
            var txtNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresUsu&quot; , &quot;'&quot; , &quot;);
            var txtApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosUsu&quot; , &quot;'&quot; , &quot;);
            var hidNombreUsu       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresUsu&quot; , &quot;'&quot; , &quot;);
            var hidApellidosUsu    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosUsu&quot; , &quot;'&quot; , &quot;);
            //
            txtIDUsu .value         = &quot;&quot;;
            txtNombreUsu .value     = &quot;&quot;;
            txtApellidosUsu .value  = &quot;&quot;; 
            hidNombreUsu.value      = &quot;&quot;; 
            hidApellidosUsu.value   = &quot;&quot;; 
    }
    
    function LimparOpe()
    {
            var txtIDOpe           = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidIDOpe&quot; , &quot;'&quot; , &quot;);
            var txtNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtNombresOpe&quot; , &quot;'&quot; , &quot;);
            var txtApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;txtApellidosOpe&quot; , &quot;'&quot; , &quot;);
            var hidNombreOpe       = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidNombresOpe&quot; , &quot;'&quot; , &quot;);
            var hidApellidosOpe    = document.getElementById(comp + &quot; , &quot;'&quot; , &quot;hidApellidosOpe&quot; , &quot;'&quot; , &quot;);
            //
            txtIDOpe.value         = &quot;&quot;;
            txtNombreOpe.value     = &quot;&quot;;
            txtApellidosOpe.value  = &quot;&quot;;
            hidApellidosOpe.value  = &quot;&quot;;
            hidApellidosOpe.value  = &quot;&quot;;
            hidNombreOpe.value = &quot;&quot;;
    }

   
    
    

    
//&lt;![CDATA[
Sys.WebForms.PageRequestManager._initialize(&quot; , &quot;'&quot; , &quot;ctl00$ContentPlaceHolder1$scmAjustes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;aspnetForm&quot; , &quot;'&quot; , &quot;, [&quot; , &quot;'&quot; , &quot;tctl00$ContentPlaceHolder1$UpdatePanel4&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;tctl00$ContentPlaceHolder1$UpdatePanel5&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;], [&quot; , &quot;'&quot; , &quot;ctl00$ContentPlaceHolder1$btnAprobar&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;], [], 90, &quot; , &quot;'&quot; , &quot;ctl00&quot; , &quot;'&quot; , &quot;);
//]]>





    
	    
	        Aprobación Ajuste de Carga
	

    
    
        
                
                    Usuario 
                    
            
                     
                    
            
            
                     
                    
            
            
                    
                
                    
                    
            
        
        
            
                Operador 
                
            
                 
                
            
            
                 
                
            
            
                
                
                
            
        
        
            
                Motivo
            
                
	- Seleccione -
	CARGA CANCELADA POR EL SISTEMA
	INGRESO POR PUERTA DE SERVICIO
	DESCUENTO ADICIONAL EN EL TORNIQUETE
	CARGA NO REALIZADA EN EL PUNTO DE VENTA
	REVERSIÓN DE MOVIMIENTO
	SISTEMA ACTUALIZA SALDO ERRADO
	ERROR AL DIGITAR VALOR DE LA CARGA
	TRASLADO DE SALDO
	VENTA CON SALDO DESCUENTA DOBLE
	SISTEMA DUPLICA CARGA
	SISTEMA REALIZA NUEVAMENTE ACTUALIZACIÓN
	INGRESO POR PUERTA DE SERVICIO - TARJETA CON DAÑO ELECTRÓNICO
	TORNIQUETE NO DESCUENTA SALDO
	DEBITO DE SALDO TARJETA AP EXTRAVIADA
	REHABILITACION MANUAL VIAJES X DINERO
	DESTRUCCION POR FALLA DAÑO O DETERIORO
	ERROR DE RECARGA INV
	TARJETA CON DAÑO ELECTRONICO O DEFECTUOSA
	CARGA IRREGULAR
	REHABILITACION MANUAL DINERO SEGUNDAS VIAS
	CADUCIDAD DE TARJETA
	DEBITO SITIADAS AFECT SESION
	DEBITO X ERROR ESCRITURA SIN AFECT SESION
	CADUCIDAD DE SALDO EN TARJETA


        
        
            
                Fecha Inicial
                (DD/MM/AAAA)
            
                
                
                
                
                
                *
            
            
            
                Fecha Fin
                (DD/MM/AAAA)
            
                
                
                
                
                
                *
                
            
            
                
                
        
    
    
    
	
            

	
            
        

    
         
        
            
	
                    
                    
                    
                    
                

                
        
        
    
        
Inicializar();
    


     

                                
                            &quot;))]</value>
      <webElementGuid>4b89fdcb-7918-4c14-8a3c-e7d68ef33963</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
