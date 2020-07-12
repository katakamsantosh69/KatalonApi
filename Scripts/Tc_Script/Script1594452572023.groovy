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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonOutput as JsonOutput
import org.openqa.selenium.Keys as Keys

//storing response
def res1 = WS.sendRequest(findTestObject('NewApi/POST_Sample_1'))

//OutPrinting
def output = JsonOutput.toJson([res1])

println(output)

//Get property value
def getpropVal = WS.getElementPropertyValue(res1, 'name')

println(getpropVal)

//verifypropvalue
def verpropval = WS.verifyElementPropertyValue(res1, 'name', getpropVal)

println(verpropval)

//geteletext
not_run: WS.getElementText(res1, 'name')

//verify element text
WS.verifyElementText(res1, 'name', 'morpheus')

