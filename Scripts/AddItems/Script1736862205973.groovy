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
import com.kms.katalon.core.testobject.ConditionType

WebUI.setText(findTestObject('Object Repository/Page_Demo Web Shop/input_Sub-Total_q'), product)

WebUI.click(findTestObject('Object Repository/Page_Demo Web Shop/input_Sub-Total_button-1 search-box-button'))

String xpath = ('//div[@class=\'product-item\' and @data-productid=\'' + id) + '\']//a'

TestObject productLink = new TestObject().addProperty('xpath', ConditionType.EQUALS, xpath)

WebUI.click(productLink)

int randomNumber = 2 + ((Math.random() * ((10 - 2) + 1)) as int)

WebUI.setText(findTestObject('Object Repository/Page_Demo Web Shop. '+ product +'/input_Qty_addtocart_'+ id +'.EnteredQuantity'), 
    randomNumber.toString())

WebUI.click(findTestObject('Object Repository/Page_Demo Web Shop. '+ product +'/input_Qty_add-to-cart-button-'+ id))
