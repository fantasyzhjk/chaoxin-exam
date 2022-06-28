mod chaoxin;
mod error;
mod exam;
mod session;
pub mod tiku;
pub use chaoxin::CheckIn;
pub use error::Result;
pub use exam::Exam;

#[cfg(test)]
mod tests {
    use crate::chaoxin::CheckIn;
    use crate::tiku;

    #[tokio::test]
    async fn upload() {
        let mut chaoxin = CheckIn::load("./courses.json").await.unwrap();
        chaoxin.load_cookies("./cookies").await.unwrap_or_default();
        chaoxin.upload_image("up_img.jpg").await.unwrap();
    }

    #[tokio::test]
    async fn fuzzy_find() {
        let list = tiku::load("JAVA程序设计.xlsx");

        let mut timu = r#"
        96. (填空题)下面程序的功能是_______________。

        public class Multiple {
        
            public static void main(String[] args) {
        
                int sum = 0;
        
                int i = 7;
        
                do{
        
                    sum = sum + i;
        
                    i = i + 7;
        
                }while(i < 50);
        
                System.out.println("1至50中是7的倍数的数值之和为：" + sum);
        
            }
        
        }
        
        "#;

        let r = regex::Regex::new(r#"(?m)[\u4e00-\u9fa5]"#).unwrap();
        let t = r
            .captures_iter(timu)
            .map(|c| c[0].to_owned())
            .collect::<String>();
        if !t.is_empty() {
            timu = &t
        }
        println!("{:?}", timu);

        let (score, ti) = tiku::fuzzy_find(&list, &timu, "填空题");
        println!("{:?}", (score, &ti));

        let ti = ti.unwrap();

        let content = if let tiku::题目类型::填空题 { content, answer: _ } = ti {
            content
        } else {
            unreachable!()
        };

        assert!(content.contains("1至50中是7的倍数的数值之和为"))
    }

    use regex::Regex;

    #[tokio::test]
    async fn refind() {
        let regex = Regex::new(r#"(?ms)<h3 class="mark_name.*?>(\d{1,}\.).*?<span class="colorShallow".*?>(.*?)</span>.*?div.*?>(.*?)</div>"#).unwrap();
        let string = "
<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
<html xmlns=\"http://www.w3.org/1999/xhtml\">
<head>
<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" />
<title>考试</title>
        <link href=\"/css/msg-note-tpl.cssx?enc=8b50d1ee718c9df5e00475d3dc3e1284\" rel=\"stylesheet\" type=\"text/css\"/>
    <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/pop.css?v=2021-0311-1412\"/>
    <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/common.css?v=2021-1206-1809\"/>
    <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/q_marking_icon.css\"/>
    <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/style.css?v=2021-1206-1809\" >
    <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/viewStudent.css?v=2020-0827-1809\"/>
    <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/newBuiltStudent.css?v=2022-0917-1140\"/>
	        <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/notAllowCopy.css\"/>
        <link rel=\"stylesheet\" type=\"text/css\" href=\"/mooc2/css/do/exam-do.css?v=2022-0617-2100\"/>
    <link href=\"/css/questionBank/questionBankUsual.css?v=2019-1120-1400\" type=\"text/css\" rel=\"stylesheet\"/></head>
<script>
	window[\"uid\"] = '118534102';
	window[\"currentTime\"] = '1656381120820';
	window[\"uploadEnc\"] = '83b2296dc9177d6693acc77704e44cd3';
</script>
<script type=\"text/javascript\">
    I18N = {
        \"confirmSub\": \"确认交卷？\",
        \"subFailed\": \"提交失败\",
        \"subSucc\": \"提交成功\",
        \"worksubmitTips1\": \"您还有未做完的\",
        \"worksubmitTips2\": \"，确认提交吗？\",
        'feedbackEmpty': '问题反馈不能为空',
        'feedbackSuccess': '反馈成功'
    };
</script><body onselectstart=\"return false\">
<style>
    .questionLi img{max-width:100%;}
</style>
<script type=\"text/javascript\" src=\"/js/common/jquery.min.js\"></script>
<script type=\"text/javascript\" src=\"/js/common/jquery-migrate.min.js\"></script><input type=\"hidden\" id=\"submitLimitTime\" name=\"submitLimitTime\" value=\"0\"/>
<input type=\"hidden\" id=\"limitTime\" name=\"limitTime\" value=\"100\"/>
<input type=\"hidden\" id=\"wordNum\" name=\"wordNum\" value=\"0\"/>
<input type=\"hidden\" id=\"qbanksystem\" name=\"qbanksystem\" value=\"0\"/>
<input type=\"hidden\" id=\"qbankbackurl\" name=\"qbankbackurl\" value=\"\"/>
<input type=\"hidden\" id=\"entryExamTime\" value=\"\">
<input type=\"hidden\" id=\"rentryExamTime\" value=\"1656381120819\">
<input type=\"hidden\" id=\"switchScreenControl\" name=\"switchScreenControl\" value=\"0\"/>
<input type=\"hidden\" id=\"snapshotMonitor\" name=\"snapshotMonitor\" value=\"0\"/>
<input type=\"hidden\" id=\"faceDetectionResult\" name=\"faceDetectionResult\" value=\"\"/>
<input type=\"hidden\" id=\"pcclientSwitchout\" name=\"pcclientSwitchout\" value=\"0\"/>
<input type=\"hidden\" id=\"saveUrl\" name=\"saveUrl\" value=\"/exam/test/reVersionSubmitTestNew\"/>
<input type=\"hidden\" id=\"monitorEnc\" name=\"monitorEnc\" value=\"893300ecd3d13980d2d0cd7861973b67\"/>
<input type=\"hidden\" id=\"uploadTimeStamp\" value=\"1656381120820\" />
<input type=\"hidden\" id=\"uploadEnc\" value=\"83b2296dc9177d6693acc77704e44cd3\" />
<input type=\"hidden\" id=\"uploadtype\" value=\"exam\" />
<input type=\"hidden\" id=\"openc\" value=\"861a8ecb2e0eb781e9662a89dbcf3c2b\" />
<input type=\"hidden\" id=\"allowPaste\" value=\"0\"/>
<input type=\"hidden\" id=\"allowDownloadAttachment\" value=\"0\"/>
<input type=\"hidden\" id=\"receiveTime\" value=\"1656380792000\"/>
<input type=\"hidden\" id=\"limitTimeType\" value=\"0\"/>
<input type=\"hidden\" id=\"singleQuesLimitTime\" value=\"\"/>
<input type=\"hidden\" id=\"forbidAnsweredAgain\" value=\"0\"/>

<input type=\"hidden\" id=\"start\" value=\"151\" />
<script type=\"text/javascript\" charset=\"utf-8\" src=\"/js/ServerHost.js?v=2020-1225-1627\"></script>
<script type=\"text/javascript\" charset=\"utf-8\" src=\"/mooc2/js/editor/ueditor.config.js?v=2021-1210-1450\"></script>
<script type=\"text/javascript\" charset=\"utf-8\" src=\"/mooc2/js/editor/ueditor.all.min.js?v=2022-0610-1600\"></script>
<script>
    function getcookie(objname){
    	var arrstr = document.cookie.split(\"; \");
    	for(var i = 0;i < arrstr.length;i ++){
    		var temp = arrstr[i].split(\"=\");
    		if(temp[0] == objname){ 
    			return unescape(temp[1]);
    		}
    	}
    }
	window[\"uid\"] = '118534102';
	window[\"currentTime\"] = '1656381120820';
	window[\"uploadEnc\"] = '83b2296dc9177d6693acc77704e44cd3';
	window.UEDITOR_CONFIG.forbidDownload = 'false';
	window.UEDITOR_CONFIG.scaleEnabled = true;
	window.UEDITOR_CONFIG.imageUrl = ServerHost.uploadDomain + \"/ueditorupload/upload\";
	window.UEDITOR_CONFIG.fileUrl = ServerHost.uploadDomain + \"/ueditorupload/attachment\";
	window.UEDITOR_CONFIG.lang = 'zh-cn';
</script><script type=\"text/javascript\" charset=\"utf-8\">
    if('0' == 0){
        window.UEDITOR_CONFIG.disableDraggable = true;
        window.UEDITOR_CONFIG.disablePasteImage = true;
    }
</script>
<script type=\"text/javascript\" src=\"/mooc2/js/pop.js?v=2021-0917-1623\"></script>
<script type=\"text/javascript\" src=\"/mooc2/js/poplayout.js\"></script>
<script src=\"/ananas/space/exam/js/enc_js_exam.js?v=1656381120821\"></script>
<script type=\"text/javascript\" src=\"/js/jquery.nicescroll.min.js\"></script>
<script type=\"text/javascript\" src=\"/mooc2/js/selectBox.js\"></script>
<script type=\"text/javascript\" src=\"/js/phone/textareaHeightAuto.js\"></script>

<script type=\"text/javascript\" src=\"/mooc2/js/exam/stu-do-exam.js?v=2022-0324-1140\"></script>
<script type=\"text/javascript\" src=\"/mooc2/js/exam/stu-exam-share.js?v=2022-0610-2100\"></script>
<script type=\"text/javascript\" src=\"/mooc2/js/exam/stu-exam-view.js?v=2020-1225-1701\"></script>
<script type=\"text/javascript\">window.SUPPORT_AUDIO_SETTING = true; window.SUPPORT_AUDIO_CONTROL = true; window.Forbid_Attachment_Title = '0';</script>

<div class=\"subNav\">
	    		<div class=\"sub-button fr\">
					<a href=\"javascript:;\" class=\"completeBtn fl\" onclick=\"topreview();\">整卷预览</a>
				</div>
	考试
</div><div class=\"het40\"></div>
<div class=\"fanyaMarking TiMu\" id=\"fanyaMarking\">
	<div class=\"fanyaMarking_left whiteBg minHet600\">
		<div class=\"padBom20 detailsHead borderBom\">
			<h2 class=\"mark_title\">124124</h2>
		
			<div class=\"infoHead\">
				<p style=\"display:block;line-height:20px;height:20px;margin-bottom:10px;margin-top:10px;\"><span>姓名: 陈芃州</span> <span>  学号: 193210103 </span></p>
				<p style=\"display:block;line-height:20px;height:20px;\"><span>题量:  356 </span>
								考试时间:<em>2022-06-28 09:46</em>至<em>2022-06-28 11:26</em>
				</p>
			</div>
								</div>
		<div class=\"mark_table\">
									
			
			<div class=\"whiteDiv questionLi singleQuesId\" data=\"882497363\"> 
							   <div >
															<h2 class=\"type_tit\">一. 单选题（共161题）</h2>
																																												<h3 class=\"mark_name colorDeep\">152. <span class=\"colorShallow\" >(单选题)</span> <div style=\"overflow:hidden;\">下列方法中,哪一个不是Applet的基本方法( )</div></h3>
				<form id=\"submitTest\" action=\"/exam/test/reVersionSubmitTestNew?keyboardDisplayRequiresUserAction=1&classId=51832179&courseId=200912172&testPaperId=2391491&testUserRelationId=66486393\" method=\"post\">
					<input type=\"hidden\" id=\"courseId\" name=\"courseId\" value=\"200912172\"/>
					<input type=\"hidden\" id=\"paperId\" name=\"paperId\" value=\"172899123\"/>
					<input type=\"hidden\" id=\"testPaperId\" name=\"testPaperId\" value=\"2391491\"/>
					<input type=\"hidden\" id=\"examCreateUserId\" name=\"examCreateUserId\" value=\"109254650\"/>
					<input type=\"hidden\" id=\"feedbackEnc\" name=\"feedbackEnc\" value=\"\"/>
					<input type=\"hidden\" id=\"testUserRelationId\" name=\"testUserRelationId\" value=\"66486393\"/>
					<input type=\"hidden\" id=\"tId\" name=\"tId\" value=\"2391491\"/>
					<input type=\"hidden\" id=\"subCount\" name=\"subCount\" value=\"\"/>
					<input type=\"hidden\" id=\"remainTime\" name=\"remainTime\" value=\"5667\"/>
					<input type=\"hidden\" id=\"encRemainTime\" name=\"encRemainTime\" value=\"5667\"/>
					<input type=\"hidden\" id=\"encLastUpdateTime\" name=\"encLastUpdateTime\" value=\"1656381120820\"/>
					<input type=\"hidden\" id=\"tempSave\" name=\"tempSave\" value=\"false\"/>
					<input type=\"hidden\" id=\"timeOver\" name=\"timeOver\" value=\"false\"/>
					<input type=\"hidden\" id=\"type\" name=\"type\" value=\"1\"/>
					<input type=\"hidden\" id=\"classId\" name=\"classId\" value=\"51832179\"/>
					<input type=\"hidden\" id=\"enc\" name=\"enc\" value=\"d65bb835b5bc8ec305331940eb5f0623\"/>
					<input type=\"hidden\" id=\"examsystem\" name=\"examsystem\" value=\"0\" />
					<input type=\"hidden\" name=\"start\" value=\"151\" />
					<input type=\"hidden\" id=\"userId\" name=\"userId\" value=\"118534102\"/>
					<input type=\"hidden\" id=\"randomOptions\" name=\"randomOptions\" value=\"false\"/>
					<input type=\"hidden\" id=\"cpi\" name=\"cpi\" value=\"109254650\">
					<input type=\"hidden\" id=\"openc\" name=\"openc\" value=\"861a8ecb2e0eb781e9662a89dbcf3c2b\">
					<input type=\"hidden\" id=\"enterPageTime\" name=\"enterPageTime\" value=\"1656381120820\"/>
					<input type=\"hidden\" name=\"questionId\" id=\"questionId\" value=\"882497363\"/>
					<input type=\"hidden\" name=\"questionScore\" id=\"questionScore\" value=\"5.0\"/>
					<input type=\"hidden\" name=\"type882497363\" value=\"0\"/>
					<input type=\"hidden\" name=\"score882497363\" value=\"5.0\"/>

										
					 						<input type=\"hidden\" id=\"answer882497363\" name=\"answer882497363\" value=\"\"/>
<div class=\"stem_answer\">
            
        
        <div class=\"clearfix answerBg\" onclick=\"addChoice(this);\">
            <span data=\"A\" qid=\"882497363\" class=\"addChoice choice882497363 num_option  fl\">A</span>
            <div class=\"fl answer_p\">init()</div>
        </div>
            
                
        <div class=\"clearfix answerBg\" onclick=\"addChoice(this);\">
            <span data=\"B\" qid=\"882497363\" class=\"addChoice choice882497363 num_option  fl\">B</span>
            <div class=\"fl answer_p\">run()</div>
        </div>
            
        
        <div class=\"clearfix answerBg\" onclick=\"addChoice(this);\">
            <span data=\"C\" qid=\"882497363\" class=\"addChoice choice882497363 num_option  fl\">C</span>
            <div class=\"fl answer_p\">stop()</div>
        </div>
            
        
        <div class=\"clearfix answerBg\" onclick=\"addChoice(this);\">
            <span data=\"D\" qid=\"882497363\" class=\"addChoice choice882497363 num_option  fl\">D</span>
            <div class=\"fl answer_p\">start()</div>
        </div>
    </div>									</form>
				</div>
									<div class=\"nextDiv\">
													 <a class=\"btnBlue btn_92 fs14\" href=\"javascript:;\" onClick=\"getTheNextQuestion(-1)\">上一题</a>						
												 	<a href=\"javascript:;\" class=\"jb_btn jb_btn_92 fs14\" onClick=\"getTheNextQuestion(1)\">下一题</a>
											</div>
				</div>
			<!--单选题 end-->
		</div>
		
	</div>
	<div class=\"fanyaMarking_right\" id=\"rightHeight\">
		<div class=\"topicNumber\" id=\"topicNumberScroll\">
            <div class=\"timeDiv\" style=\"display: none;\"><i><img src=\"/mooc2/images/time.png\"></i><span id=\"timer\"></span></div>
										<div class=\"topicNumber_checkbox colorDeep fs14\"><span class=\"numRight fr\"></span>1. 单选题</div>
				<ul class=\"topicNumber_list clearfix\">
																						<li  class=\"\"   onclick=\"getTheQuestionByStart(0);\" >1</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(1);\" >2</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(2);\" >3</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(3);\" >4</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(4);\" >5</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(5);\" >6</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(6);\" >7</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(7);\" >8</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(8);\" >9</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(9);\" >10</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(10);\" >11</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(11);\" >12</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(12);\" >13</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(13);\" >14</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(14);\" >15</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(15);\" >16</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(16);\" >17</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(17);\" >18</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(18);\" >19</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(19);\" >20</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(20);\" >21</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(21);\" >22</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(22);\" >23</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(23);\" >24</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(24);\" >25</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(25);\" >26</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(26);\" >27</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(27);\" >28</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(28);\" >29</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(29);\" >30</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(30);\" >31</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(31);\" >32</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(32);\" >33</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(33);\" >34</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(34);\" >35</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(35);\" >36</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(36);\" >37</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(37);\" >38</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(38);\" >39</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(39);\" >40</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(40);\" >41</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(41);\" >42</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(42);\" >43</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(43);\" >44</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(44);\" >45</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(45);\" >46</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(46);\" >47</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(47);\" >48</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(48);\" >49</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(49);\" >50</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(50);\" >51</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(51);\" >52</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(52);\" >53</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(53);\" >54</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(54);\" >55</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(55);\" >56</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(56);\" >57</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(57);\" >58</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(58);\" >59</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(59);\" >60</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(60);\" >61</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(61);\" >62</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(62);\" >63</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(63);\" >64</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(64);\" >65</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(65);\" >66</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(66);\" >67</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(67);\" >68</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(68);\" >69</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(69);\" >70</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(70);\" >71</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(71);\" >72</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(72);\" >73</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(73);\" >74</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(74);\" >75</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(75);\" >76</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(76);\" >77</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(77);\" >78</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(78);\" >79</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(79);\" >80</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(80);\" >81</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(81);\" >82</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(82);\" >83</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(83);\" >84</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(84);\" >85</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(85);\" >86</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(86);\" >87</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(87);\" >88</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(88);\" >89</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(89);\" >90</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(90);\" >91</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(91);\" >92</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(92);\" >93</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(93);\" >94</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(94);\" >95</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(95);\" >96</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(96);\" >97</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(97);\" >98</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(98);\" >99</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(99);\" >100</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(100);\" >101</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(101);\" >102</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(102);\" >103</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(103);\" >104</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(104);\" >105</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(105);\" >106</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(106);\" >107</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(107);\" >108</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(108);\" >109</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(109);\" >110</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(110);\" >111</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(111);\" >112</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(112);\" >113</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(113);\" >114</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(114);\" >115</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(115);\" >116</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(116);\" >117</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(117);\" >118</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(118);\" >119</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(119);\" >120</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(120);\" >121</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(121);\" >122</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(122);\" >123</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(123);\" >124</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(124);\" >125</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(125);\" >126</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(126);\" >127</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(127);\" >128</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(128);\" >129</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(129);\" >130</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(130);\" >131</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(131);\" >132</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(132);\" >133</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(133);\" >134</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(134);\" >135</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(135);\" >136</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(136);\" >137</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(137);\" >138</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(138);\" >139</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(139);\" >140</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(140);\" >141</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(141);\" >142</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(142);\" >143</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(143);\" >144</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(144);\" >145</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(145);\" >146</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(146);\" >147</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(147);\" >148</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(148);\" >149</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(149);\" >150</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(150);\" >151</li>
																			<li  class=\" current \"   onclick=\"getTheQuestionByStart(151);\" >152</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(152);\" >153</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(153);\" >154</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(154);\" >155</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(155);\" >156</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(156);\" >157</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(157);\" >158</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(158);\" >159</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(159);\" >160</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(160);\" >161</li>
																</ul>
							<div class=\"topicNumber_checkbox colorDeep fs14\"><span class=\"numRight fr\"></span>2. 填空题</div>
				<ul class=\"topicNumber_list clearfix\">
																						<li  class=\"\"   onclick=\"getTheQuestionByStart(161);\" >1</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(162);\" >2</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(163);\" >3</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(164);\" >4</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(165);\" >5</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(166);\" >6</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(167);\" >7</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(168);\" >8</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(169);\" >9</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(170);\" >10</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(171);\" >11</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(172);\" >12</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(173);\" >13</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(174);\" >14</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(175);\" >15</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(176);\" >16</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(177);\" >17</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(178);\" >18</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(179);\" >19</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(180);\" >20</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(181);\" >21</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(182);\" >22</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(183);\" >23</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(184);\" >24</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(185);\" >25</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(186);\" >26</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(187);\" >27</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(188);\" >28</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(189);\" >29</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(190);\" >30</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(191);\" >31</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(192);\" >32</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(193);\" >33</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(194);\" >34</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(195);\" >35</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(196);\" >36</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(197);\" >37</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(198);\" >38</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(199);\" >39</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(200);\" >40</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(201);\" >41</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(202);\" >42</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(203);\" >43</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(204);\" >44</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(205);\" >45</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(206);\" >46</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(207);\" >47</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(208);\" >48</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(209);\" >49</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(210);\" >50</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(211);\" >51</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(212);\" >52</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(213);\" >53</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(214);\" >54</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(215);\" >55</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(216);\" >56</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(217);\" >57</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(218);\" >58</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(219);\" >59</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(220);\" >60</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(221);\" >61</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(222);\" >62</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(223);\" >63</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(224);\" >64</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(225);\" >65</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(226);\" >66</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(227);\" >67</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(228);\" >68</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(229);\" >69</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(230);\" >70</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(231);\" >71</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(232);\" >72</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(233);\" >73</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(234);\" >74</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(235);\" >75</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(236);\" >76</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(237);\" >77</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(238);\" >78</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(239);\" >79</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(240);\" >80</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(241);\" >81</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(242);\" >82</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(243);\" >83</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(244);\" >84</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(245);\" >85</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(246);\" >86</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(247);\" >87</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(248);\" >88</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(249);\" >89</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(250);\" >90</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(251);\" >91</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(252);\" >92</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(253);\" >93</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(254);\" >94</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(255);\" >95</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(256);\" >96</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(257);\" >97</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(258);\" >98</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(259);\" >99</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(260);\" >100</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(261);\" >101</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(262);\" >102</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(263);\" >103</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(264);\" >104</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(265);\" >105</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(266);\" >106</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(267);\" >107</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(268);\" >108</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(269);\" >109</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(270);\" >110</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(271);\" >111</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(272);\" >112</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(273);\" >113</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(274);\" >114</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(275);\" >115</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(276);\" >116</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(277);\" >117</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(278);\" >118</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(279);\" >119</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(280);\" >120</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(281);\" >121</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(282);\" >122</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(283);\" >123</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(284);\" >124</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(285);\" >125</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(286);\" >126</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(287);\" >127</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(288);\" >128</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(289);\" >129</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(290);\" >130</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(291);\" >131</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(292);\" >132</li>
																</ul>
							<div class=\"topicNumber_checkbox colorDeep fs14\"><span class=\"numRight fr\"></span>3. 判断题</div>
				<ul class=\"topicNumber_list clearfix\">
																						<li  class=\"\"   onclick=\"getTheQuestionByStart(293);\" >1</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(294);\" >2</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(295);\" >3</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(296);\" >4</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(297);\" >5</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(298);\" >6</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(299);\" >7</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(300);\" >8</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(301);\" >9</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(302);\" >10</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(303);\" >11</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(304);\" >12</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(305);\" >13</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(306);\" >14</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(307);\" >15</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(308);\" >16</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(309);\" >17</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(310);\" >18</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(311);\" >19</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(312);\" >20</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(313);\" >21</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(314);\" >22</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(315);\" >23</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(316);\" >24</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(317);\" >25</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(318);\" >26</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(319);\" >27</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(320);\" >28</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(321);\" >29</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(322);\" >30</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(323);\" >31</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(324);\" >32</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(325);\" >33</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(326);\" >34</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(327);\" >35</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(328);\" >36</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(329);\" >37</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(330);\" >38</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(331);\" >39</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(332);\" >40</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(333);\" >41</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(334);\" >42</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(335);\" >43</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(336);\" >44</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(337);\" >45</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(338);\" >46</li>
																</ul>
							<div class=\"topicNumber_checkbox colorDeep fs14\"><span class=\"numRight fr\"></span>4. 论述题</div>
				<ul class=\"topicNumber_list clearfix\">
																						<li  class=\"\"   onclick=\"getTheQuestionByStart(339);\" >1</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(340);\" >2</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(341);\" >3</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(342);\" >4</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(343);\" >5</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(344);\" >6</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(345);\" >7</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(346);\" >8</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(347);\" >9</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(348);\" >10</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(349);\" >11</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(350);\" >12</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(351);\" >13</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(352);\" >14</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(353);\" >15</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(354);\" >16</li>
																			<li  class=\"\"   onclick=\"getTheQuestionByStart(355);\" >17</li>
																</ul>
					</div>
	</div>
</div>
<div class=\"maskDiv\" style=\"display:none;z-index:2001;\"  id=\"timeOverSubmitConfirmPop\">
	<div class=\"popDiv wid440 Marking\" style=\"left:39%;top:40%;width:480px;height:300px\">
			<div class=\"popHead\">
				<p class=\"fl fs18 colorDeep\">提示</p>
			</div>
			<div class=\"het62\"></div>
			<p class=\"popWord fs16 colorIn\" style=\"margin: 10px 2px;\">作答时间耗尽，试卷已提交</p>
			<p class=\"popWord fs16 colorIn\" style=\"margin: 2px 2px;\">试卷领取时间：2022年06月28日09:46</p>
			<p class=\"popWord fs16 colorIn\" style=\"margin: 10px 2px;\">考试用时：<span class=\"consumeMinutes\">100</span>分钟</p>

			<div class=\"popBottom\">
				<a href=\"javascript:\" class=\"jb_btn jb_btn_92 fr fs14 confirmClose\" onclick=\"\">知道了</a>
			</div>
			<div class=\"het72\"></div>
	</div>
</div>


<div class=\"maskDiv\" style=\"display:none;z-index:2000;\"  id=\"submitConfirmPop\">
	<div class=\"popDiv wid440 Marking\" style=\"left:39%;top:30%;width:450px;min-height:200px\">
			<div class=\"popHead\">
				<p class=\"fl fs18 colorDeep\">提示</p>
				<a href=\"javascript:;\" class=\"popClose fr\"><img src=\"/mooc2/images/popClose.png\" onclick=\"$('#submitConfirmPop').fullFadeOut();\"></a>
			</div>
			<div class=\"het62\"></div>
			<p class=\"popWord fs16 colorIn\" style=\"margin: 30px 2px;\">确认交卷？</p>

			<div class=\"popBottom\">
				<a href=\"javascript:\" class=\"jb_btn jb_btn_92 fr fs14 confirm\" onclick=\"\">确定</a>
				<a href=\"javascript:\" class=\"btnBlue btn_92_cancel fr fs14\" onclick=\"$('#submitConfirmPop').fullFadeOut();\">取消</a>
			</div>
			<div class=\"het72\"></div>
	</div>
</div>


<div class=\"maskDiv\" style=\"display:none;z-index:1000;\"  id=\"audioLimitTimesWin\">
	<div class=\"popSetDiv wid440\">
			<div class=\"popHead RadisTop\">
				<a href=\"javascript:;\" class=\"popClose fr\" onclick=\"$('#audioLimitTimesWin').fullFadeOut();\"><img src=\"/mooc2/images/popClose.png\" /></a>
				<p class=\"fl fs18 color1\">提示</p>
			</div>
			<div class=\"het62\"></div>
			<p class=\"popWord fs16 color2 audioLimitTimesTip\">此附件仅支持打开 <span></span> 次，你已打开 <span></span> 次，不能再次打开</p>
			<div class=\"popBottom RadisBom\">
				<a href=\"javascript:;\" class=\"jb_btn jb_btn_92 fr fs14\" onclick=\"$('#audioLimitTimesWin').fullFadeOut();\">知道了</a>
			</div>
			<div class=\"het72\"></div>
	</div>
</div>

<div class=\"maskDiv popMoveShowHide\" id=\"confirmEnterWin\" style=\"display:none;z-index:1000;\">
	<div class=\"popDiv wid440 popMove\">
		<div class=\"popHead\">
			<a href=\"javascript:;\" class=\"popClose fr\"><img src=\"/mooc2/images/popClose.png\" onclick=\"$('#confirmEnterWin').fullFadeOut();\"/></a>
			<p class=\"fl fs18 colorDeep\" style=\"font-size:18px;\">提示</p>
		</div>
		<div class=\"het62\"></div>
		<div class=\"readPad\" style=\"padding-bottom:0px;\">
			<div class=\" tip\" style=\"line-height:26px;font-size:16px;min-height: 140px;width:100%;\"></div>
		</div>
		<div class=\"popBottom\">
			<a href=\"javascript:;\" class=\"jb_btn jb_btn_92 fr fs14 confirm\" onclick=\"\">进入考试</a>
			<a href=\"javascript:;\" class=\"btnBlue btn_92_cancel fr fs14\" onclick=\"$('#confirmEnterWin').fullFadeOut();\" style=\"width:88px;\">取消</a>
		</div>
		<div class=\"het72\"></div>
	</div>
</div>


<div class=\"maskDiv\" style=\"display:none;z-index:1000;\"  id=\"multiTerminalWin\">
	<div class=\"popDiv wid440 Marking\" style=\"left:50%;top:40%;width:480px;height:300px;margin-left:-240px;\">
			<div class=\"popHead\">
				<p class=\"fl fs18 colorDeep\">提示</p>
			</div>
			<div class=\"het62\"></div>
			<p class=\"popWord fs16 colorIn\" style=\"margin:18px 2px;\"></p>
			<div class=\"popBottom\">
				<a href=\"javascript:\" class=\"jb_btn jb_btn_92 fr fs14 confirmClose\" onclick=\"\">重新进入</a>
				<a href=\"javascript:;\" class=\"btnBlue btn_92_cancel fr fs14 cancel\" onclick=\"$('#confirmEnterWin').fullFadeOut();\" style=\"width:88px;\">取消</a>
			</div>
			<div class=\"het72\"></div>
	</div>
</div>



<div class=\"maskDiv\" style=\"display:none;z-index:1000;\"  id=\"examTipsPop\">
	<div class=\"popDiv wid440 Marking\" style=\"left:39%;top:40%;width:480px;height:300px\">
			<div class=\"popHead\">
				<p class=\"fl fs18 colorDeep\">提示</p>
			</div>
			<div class=\"het62\"></div>
			<div class=\"popWord fs16 colorIn\" style=\"margin: 6px 2px;overflow: auto;height:160px;word-break: break-all;\"></div>
			<div class=\"popBottom\">
				<a href=\"javascript:\" class=\"jb_btn jb_btn_92 fr fs14 confirmClose\" onclick=\"$('#examTipsPop').fullFadeOut();\">知道了</a>
			</div>
			<div class=\"het72\"></div>
	</div>
</div>

<div class=\"maskDiv\" style=\"display:none;z-index:1000;\"  id=\"singleQuesLimitTimePop\">
	<div class=\"popDiv wid440 Marking\" style=\"left:39%;top:40%;width:480px;height:200px\">
			<div class=\"popHead\">
				<p class=\"fl fs18 colorDeep\">提示</p>
			</div>
			<div class=\"het62\"></div>
			<div class=\"popWord fs16 colorIn\" style=\"margin: 6px 2px;overflow: auto;height:160px;word-break: break-all;position:relative;top:20px;\"> 本题作答时间已用完，将进入下一题 </div>
			<div class=\"popBottom\">
				<a href=\"javascript:\" class=\"jb_btn jb_btn_92 fr fs14 confirmClose\" onclick=\"singleQuesLimitTimeConfirm();\"> 确定 </a>
			</div>
			<div class=\"het72\"></div>
	</div>
</div>
<script>
	$(document).keydown(function(event){
		if ((event.altKey)&& ((event.keyCode==37)|| (event.keyCode==39))) {
			event.returnValue=false;
			return false;
		}

		if(event.keyCode==116){
			return false;
		}

		if((event.ctrlKey) && (event.keyCode==82)){
			return false;
		}

		if(event.keyCode==13){
			event.preventDefault();
		}
	});

	var maxSecond = 59;
	maxtime = 5667;
	var limitTime = 100 * 60;
	var testPaperStatus = \"1\";

	if(maxtime >= 0) {
		timers = setInterval(\"CountDown()\",1000);
		function CountDown(){
			if(maxtime>0){
				minutes = Math.floor(maxtime/60);
				if(minutes < 10) {
					minutes = \"0\" + minutes;
				}
				seconds = Math.floor(maxtime%60);
				if(seconds < 10) {
					seconds = \"0\" + seconds;
				}
				msg = minutes +\"' \"+seconds+\"''\";

				$(\".timeDiv\").show();
				$(\"#timer\").html( msg);
				$(\"#remainTime\").val(maxtime);
				--maxtime;
			} else{
				$(\"#timeOver\").val(true);
				clearInterval(timers);
				timeOverSubmitTest();
			}
		}
	}

	$(document).ready(function () {
    	var snapshotMonitor = $('#snapshotMonitor').val();
    	if (snapshotMonitor == 1) {
    		var entryExamTime = $('#entryExamTime').val();
    		var rentryExamTime = $('#rentryExamTime').val();
    		var data = {};
    		try{
        		var faceDetectionResult = $('#faceDetectionResult').val();
        		if (faceDetectionResult && faceDetectionResult != '') {
        			data = JSON.parse(faceDetectionResult);
        		}
    		}catch(err){}
    		if (entryExamTime && entryExamTime > 0) {
    			data.entryTime = entryExamTime;
    			entryExamLog(data);
    		} else if (rentryExamTime && rentryExamTime > 0) {
    			data.rentryTime = rentryExamTime;
    			rentryExamLog(data);
    		}
    	    // openExamClientFaceMonitor('-1', '0', '{\"classId\":\"51832179\", \"answerId\":\"66486393\"}') ;
			openExamClientScreenAndCapture('-1', '0', '{\"classId\":\"51832179\", \"answerId\":\"66486393\"}');
    	}
    	var pcclientSwitchout = $('#pcclientSwitchout').val();
    	if(pcclientSwitchout == 1){
    	    openExamClientScreenCutting();
    	}
	});

	function getTheNextQuestion(n){
		submitForm(true, false, function(data) {
			var array=data.split(\"|\");
			var s=151+n;
			var lastUpdateTime=array[0];
			var remainTime=array[1];
			var enc=array[2];
			window.location.href=\"/exam/test/reVersionTestStartNew?keyboardDisplayRequiresUserAction=1&getTheNextQuestion=1&courseId=200912172&classId=51832179\"+
					\"&tId=2391491&id=66486393&p=1&start=\"+s+ \"&remainTimeParam=\"+remainTime+\"&relationAnswerLastUpdateTime=\"+lastUpdateTime+\"&enc=\"+enc
					+\"&qbanksystem=0&qbankbackurl=&cpi=109254650&openc=861a8ecb2e0eb781e9662a89dbcf3c2b&newMooc=true\";
		});
	}

	function getTheQuestionByStart(n){
		submitForm(true, false, function(data) {
			var array=data.split(\"|\");
			var lastUpdateTime=array[0];
			var remainTime=array[1];
			var enc=array[2];
			window.location.href=\"/exam/test/reVersionTestStartNew?keyboardDisplayRequiresUserAction=1&courseId=200912172&classId=51832179&tId=2391491&id=66486393&p=1&start=\"+n+
					\"&qbanksystem=0&qbankbackurl=&remainTimeParam=\"+remainTime+\"&relationAnswerLastUpdateTime=\"+lastUpdateTime+\"&enc=\"+enc+\"&cpi=109254650&openc=861a8ecb2e0eb781e9662a89dbcf3c2b&newMooc=true\";
		});
	}

	$(function(){
	
	    checkStartSingleQuesLimitTimeTimer();
		checkBrowerBack();
		
	    $(\".optionsCon\").niceScroll({
			cursorborder : \"\",
			cursorwidth : \"8px\",
			cursorcolor : \"#E6ECF5\",
			boxzoom : false
		});
		$('textarea').autoHeight({len:53});

		 $(document).ready(function() {
			$(\".programAswer\").niceScroll({cursorborder:\"\",cursorwidth:\"8px\", cursorcolor:\"#E6ECF5\",boxzoom:false});
			$(\".optionsCon\").niceScroll({cursorborder:\"\", cursorwidth:\"8px\", cursorcolor:\"#E6ECF5\",boxzoom:false});
		  });
	});
		
	$(document).bind(\"visibilitychange\",function(e){
         checkRemainTime(document.visibilityState);
    });
</script>
<script src=\"/space/work/js/preview-attach.js?v=2022-0126-1809\"></script><link href=\"/css/work/viewer.min.css?v=2021-0830-1700\" rel=\"stylesheet\" type=\"text/css\" />
<script type=\"text/javascript\" src=\"/js/jquery.md5.js\"></script>
<script type=\"text/javascript\" src=\"/space/work/js/viewer-jquery.min.js?v=2021-0706-1800\"></script>
<script type=\"text/javascript\">
try{
	$(function(){
		var imgList = $(\".TiMu\").find(\"div img:not(.workAttach img, .attach img, .attachNew img, .stuAnswerArea img, .popClose img, .ans-formula-moudle)\");
		for (var i = 0, len = imgList.size(); i < len; i++) {
			 var src = imgList.eq(i).attr(\"src\");
			 if(src){
    			var index = src.indexOf(\"375_1024\");
    			if (index != -1) {
    				src = src.replace(\"375_1024\", \"origin\");
    			}
                 var index2 = src.indexOf(\"750_1024\");
                 if (index2 != -1) {
                     src = src.replace(\"750_1024\", \"origin\");
                 }
    			imgList.eq(i).attr(\"data-original\", src);
		   }
		}
        $(\".TiMu\").find(\"div img:not(.workAttach img, .attach img, .attachNew img, .stuAnswerArea img, .popClose img)\").viewer({
			url : 'data-original',
		});
	})
  }catch(error){}
</script>	</body>
</html>
";

        // result will be an iterator over tuples containing the start and end indices for each match in the string
        let result = regex.captures_iter(string);

        for mat in result {
            println!("{:?}", mat);
        }
    }
}
