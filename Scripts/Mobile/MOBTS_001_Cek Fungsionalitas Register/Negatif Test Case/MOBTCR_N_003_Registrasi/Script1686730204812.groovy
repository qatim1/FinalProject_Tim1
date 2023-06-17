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

Mobile.startApplication('C:\\Users\\adria\\Downloads\\DemoAppV2.apk', true)

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.TextView - Login Here'), 0)

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.TextView - Register, now'), 0)

Mobile.setText(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.Nama'), 'Abdul Budi', 
    0)

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.TextView -'), 0)

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.TextView - 2016'), 0)

Mobile.scrollToText('2004')

Mobile.scrollToText('1999')

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.TextView - 1999'), 0)

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.ImageButton'), 0)

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.view.View - 14'), 0)

Mobile.tap(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.Button - OK'), 0)

Mobile.setText(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.Email'), 'testcdid123@gmail.com', 
    0)

Mobile.setText(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.Whatsapp'), '089637557848', 
    0)

Mobile.setEncryptedText(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.Katasandi'), 
    'R2dZ4hvJ2ujoDGjQ2cClHw==', 0)

Mobile.setEncryptedText(findTestObject('Object Repository/Adrian Ramdo Firmansyah/Mobile/android.widget.KonfirmasiKatasandi'), 
    'R2dZ4hvJ2ujoDGjQ2cClHw==', 0)

Mobile.verifyElementAttributeValue(findTestObject('Adrian Ramdo Firmansyah/Mobile/android.view.ViewGroup'), 'clickable', 
    'false', 0)

Mobile.closeApplication()

