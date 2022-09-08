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

WebUI.getAttribute(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Usuarios - Consultas -  Extracto del Usuario Persona Natural - Detalles/span_1.009.010.785_atcexd', 
        [('variable') : Saldo_a_debitar]), '')

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
    '8516297')

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro Medelln -  Consulta de Persona Natural/input_REGISTRO BLOQUEADO_btnBuscar_cpn'))

WebUI.waitForPageLoad(1)

WebUI.click(findTestObject('Consulta y ajuste de saldo/Page_Metro Medelln -  Consulta de Persona Natural/img_cpn'))

WebUI.switchToWindowUrl('https://qabanco.civica.com.co/Civica/Adquisicion/PuntodeVenta/frmAjusteCargaDet.aspx')

WebUI.focus(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/input_Valor_ctl00ContentPlaceHolder1txtValor_pdvab'))

WebUI.setText(findTestObject('Consulta y ajuste de saldo/Page_Metro de Medelln - Adquisicion - Punto de Venta -  Ajuste de Carga/input_Valor_ctl00ContentPlaceHolder1txtValor_pdvab'), 
    Saldo_a_debitar)

