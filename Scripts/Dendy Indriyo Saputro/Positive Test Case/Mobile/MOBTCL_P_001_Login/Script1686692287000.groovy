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

Mobile.setText(findTestObject('Dendy Indriyo Saputro/Mobile/Login/android.widget.EditText - Email Field - Login'), 'testcdid123@gmail.com', 
    0)

Mobile.setEncryptedText(findTestObject('Dendy Indriyo Saputro/Mobile/Login/android.widget.EditText - Password Field - Login'), 
    '/1ZObZjf6SXVzGwZ9Vhvcg==', 0)

Mobile.tap(findTestObject('Dendy Indriyo Saputro/Mobile/Login/android.view.ViewGroup - Login Button - Login'), 0)

Mobile.delay(1, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Dendy Indriyo Saputro/Mobile/Main/android.widget.TextView - Profile Icon - Main'), 0)

Mobile.delay(2, FailureHandling.STOP_ON_FAILURE)

Mobile.verifyElementText(findTestObject('Dendy Indriyo Saputro/Mobile/Profile/android.widget.TextView - testcdid123gmail.com'), 
    'testcdid123@gmail.com')

Mobile.delay(2, FailureHandling.STOP_ON_FAILURE)

Mobile.closeApplication()

