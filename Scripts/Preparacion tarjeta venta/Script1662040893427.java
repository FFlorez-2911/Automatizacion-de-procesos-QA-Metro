import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import java.lang.String as String
import java.lang.System as System

WebUI.openBrowser('https://qabanco.civica.com.co/civica/index.aspx')

WebUI.waitForPageLoad(0)

WebUI.maximizeWindow()

WebUI.focus(findTestObject('Inicio de sesion - Cívica Calidad/Inicio de sesion_Civica Calidad/input_Cdula usuario_txtLogin'))

WebUI.setText(findTestObject('Inicio de sesion - Cívica Calidad/Inicio de sesion_Civica Calidad/input_Cdula usuario_txtLogin'), 
    '1028015407')

WebUI.focus(findTestObject('Inicio de sesion - Cívica Calidad/Inicio de sesion_Civica Calidad/input_Contrasea_txtSena'))

WebUI.setText(findTestObject('Inicio de sesion - Cívica Calidad/Inicio de sesion_Civica Calidad/input_Contrasea_txtSena'), 
    '12345678')

WebUI.click(findTestObject('Inicio de sesion - Cívica Calidad/Inicio de sesion_Civica Calidad/input_Contrasea_btnOk'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Pagina inicial/Page_Metro de Medelln - Sea bienvenido/em_Atencin_bv'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln -  Atencin/a_Consultas_at'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Atencin -  Consultas/a_Extracto PN_atc'))

WebUI.focus(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Atencin - Consultas -  Extracto del Usuario Persona Natural/input__ctl00ContentPlaceHolder1txtTarjeta_atcex'))

WebUI.setText(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Atencin - Consultas -  Extracto del Usuario Persona Natural/input__ctl00ContentPlaceHolder1txtTarjeta_atcex'), 
    '8518211')

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Atencin - Consultas -  Extracto del Usuario Persona Natural/input__ctl00ContentPlaceHolder1btnBuscar_atcex'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Usuarios - Consultas -  Extracto del Usuario Persona Natural - Detalles/input__ctl00ContentPlaceHolder1btnBuscar_atcexd'))

WebUI.waitForPageLoad(0)

saldodebito = WebUI.getText(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Usuarios - Consultas -  Extracto del Usuario Persona Natural - Detalles/span_1.009.010.785_atcexd'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Usuarios - Consultas -  Extracto del Usuario Persona Natural - Detalles/em_Adquisicin_atcexd'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medellin -  Adquisicin/a_Punto de Venta_adq'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Ajuste de Carga/a_Ajuste de Carga_pdva'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Ajuste de Carga/a_Nuevo Registro'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/input_Apellidos_ctl00ContentPlaceHolder1btnBuscarUsu_pdvab'))

WebUI.waitForPageLoad(0)

WebUI.switchToWindowUrl('https://qabanco.civica.com.co/Civica/Usuarios/Registros/frmUsuariosPNTarjetaPopup.aspx?nombre=&cedula=&tarjeta=')

WebUI.focus(findTestObject('Consulta y ajuste de saldo/Page_Metro Medelln -  Consulta de Persona Natural/input_Nm. Tarjeta_txtNumeroTarjeta_cpn'))

WebUI.setText(findTestObject('Consulta y ajuste de saldo/Page_Metro Medelln -  Consulta de Persona Natural/input_Nm. Tarjeta_txtNumeroTarjeta_cpn'), 
    '8518211')

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro Medelln -  Consulta de Persona Natural/input_REGISTRO BLOQUEADO_btnBuscar_cpn'))

WebUI.waitForPageLoad(0)

WebUI.doubleClick(findTestObject('Consulta y ajuste de saldo/Page_Metro Medelln -  Consulta de Persona Natural/img_cpn'))

WebUI.switchToWindowUrl('https://qabanco.civica.com.co/Civica/Adquisicion/PuntodeVenta/frmAjusteCargaDet.aspx')

WebUI.focus(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/input_Valor_ctl00ContentPlaceHolder1txtValor_pdvab'))

WebUI.setText(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/input_Valor_ctl00ContentPlaceHolder1txtValor_pdvab'), 
    saldodebito)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/select_- Seleccione - Motivo_pdvab'))

WebUI.sendKeys(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/select_- Seleccione - Motivo_pdvab'), 
    Keys.chord(Keys.ARROW_DOWN))

WebUI.sendKeys(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/select_- Seleccione - Motivo_pdvab'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/textarea_Caracteres restantes  1000_ctl00ContentPlaceHolder1txtDescripcion_pdvab'))

WebUI.setText(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/textarea_Caracteres restantes  1000_ctl00ContentPlaceHolder1txtDescripcion_pdvab'), 
    'Prueba venta de tarjetas')

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/input_Operador Modificacin_ctl00ContentPlaceHolder1btnConfirmar_pdvab'))

WebUI.switchToWindowUrl('https://qabanco.civica.com.co/Civica/Includes/frmMensagem.aspx')

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro Medelln -  Mensaje/input_Operacin efectuada con xito_btnOk_cpn'))

WebUI.switchToWindowUrl('https://qabanco.civica.com.co/Civica/Adquisicion/PuntodeVenta/frmAjusteCargaDet.aspx?Cod=0')

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/a_Aprobacin Ajuste de Carga_pdvab'))

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Aprobacin Ajuste de Carga/select_- Seleccione - Motivo Aprobacion_adqa'))

WebUI.sendKeys(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Aprobacin Ajuste de Carga/select_- Seleccione - Motivo Aprobacion_adqa'), 
    Keys.chord(Keys.ARROW_DOWN))

WebUI.sendKeys(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Aprobacin Ajuste de Carga/select_- Seleccione - Motivo Aprobacion_adqa'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Aprobacin Ajuste de Carga/input__ctl00ContentPlaceHolder1btnBuscar_adqa'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Aprobacin Ajuste de Carga/input__ctl00ContentPlaceHolder1btnBuscar_adqa'))

WebUI.waitForPageLoad(0)

WebUI.check(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Aprobacin Ajuste de Carga/input_PENDIENTE_ctl00ContentPlaceHolder1gvAjustesctl02chkOpciones_adqa'))

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicin - Punto de Venta -  Aprobacin Ajuste de Carga/input_registro(s) encontrado(s)_ctl00ContentPlaceHolder1btnAprobar_adqa'))

