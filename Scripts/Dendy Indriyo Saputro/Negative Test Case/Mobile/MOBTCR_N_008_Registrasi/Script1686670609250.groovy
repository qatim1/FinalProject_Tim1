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

Mobile.startApplication('D:\\APK\\DemoAppV2.apk', true)

Mobile.tap(findTestObject('Dendy Indriyo Saputro/Mobile/Main/android.widget.TextView - Login Here Button'), 0)

Mobile.tap(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.widget.TextView - Register, now'), 0)

Mobile.setText(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.widget.EditText - Nama Field'), 'Abdul Budi', 
    0)

Mobile.tap(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.widget.EditText -Tanggal Lahir Field'), 0)

Mobile.tap(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.widget.Button - OK (After Tanggal Lahir)'), 0)

Mobile.setText(findTestObject('Object Repository/Dendy Indriyo Saputro/Mobile/Register/android.widget.EditText - WhatsApp Field'), 
    '089637557848', 0)

Mobile.setEncryptedText(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.widget.EditText - Kata Sandi Field'), 
    '/1ZObZjf6SWlTY4xbVQyPA==', 0)

Mobile.setEncryptedText(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.widget.EditText - Konfirmasi Kata Sandi Field'), 
    '/1ZObZjf6SWlTY4xbVQyPA==', 0)

Mobile.tap(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.widget.CheckBox - Term and Condition'), 0)

Mobile.verifyElementAttributeValue(findTestObject('Dendy Indriyo Saputro/Mobile/Register/android.view.ViewGroup - Daftar Button'), 
    'clickable', 'false', 0)

Mobile.delay(2, FailureHandling.STOP_ON_FAILURE)

Mobile.closeApplication()

