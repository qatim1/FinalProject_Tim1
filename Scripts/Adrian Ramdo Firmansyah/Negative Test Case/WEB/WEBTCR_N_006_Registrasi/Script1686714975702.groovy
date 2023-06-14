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

WebUI.openBrowser('https://demo-app.online/')

WebUI.click(findTestObject('Adrian Ramdo Firmansyah/Website/Main_Page/button_Buat Akun'))

WebUI.setText(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Field_nama'), 'Abdul Budi')

WebUI.setText(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Field_Tanggal lahir'), '30-Aug-2020')

WebUI.setText(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Field_Email'), 'testcdid123@gmail.com')

WebUI.setText(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Field_Whatsapp'), '089637557848')

WebUI.setEncryptedText(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Field_Password'), '/1ZObZjf6SXVzGwZ9Vhvcg==')

WebUI.setEncryptedText(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Field_Konfirmasi_Password'), '/1ZObZjf6SXVzGwZ9Vhvcg==')

WebUI.click(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Checkbox_Persetujuan'))

String expectedDate = '31-Dec-2016'

String actualDate = WebUI.getAttribute(findTestObject('Adrian Ramdo Firmansyah/Website/Page_Buat_Akun/Field_Tanggal lahir'), 
    'value')

WebUI.verifyMatch(actualDate, expectedDate, true)

WebUI.closeBrowser()

