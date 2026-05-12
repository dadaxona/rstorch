py -m venv venv
venv\Scripts\activate
venv\Scripts\Activate.ps1
python.exe -m pip install --upgrade pip
pip list
pip install django
django-admin startproject Project .
py manage.py startapp project
pip install djangorestframework django-cors-headers
python manage.py makemigrations
python manage.py migrate
python manage.py runserver

pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu124

YOLO, cv2
pip install opencv-python
pip install ultralytics requests opencv-python

YOLO train command
yolo task=detect mode=train model=yolov8n.pt data=C:/Users/abdul/Desktop/XodimAI/Analiz/dataset/data.yaml epochs=50 imgsz=640

pip install -r requirements.txt

# Create
    Node.js ---> Staff.create({ // columns // })
    Python ---> staff = Staff.objects.create(firstname="Ali", lastname="Valiyev")

# Create
    Node.js
        const data = {"firstname": "Ali", "lastname": "Valiyev"}
        Staff.create({ ...data })

    Python
        data = {"firstname": "Ali", "lastname": "Valiyev"}
        staff = Staff.objects.create(**data)

# FindById
    Node.js
        Staff.findByPk(id)

    Python
        staff = Staff.objects.get(pk=req_id)

# findAll
    Node.js
        Staff.findAll({ where: {id: id} })

    Python
        staff_list = Staff.objects.filter(id=req_id)

# findOne
    Node.js
        Staff.findOne({ where: {id: id} })

    Python
        staff = Staff.objects.filter(id=req_id).first()

# update
    Node.js
        Staff.update({ // columns // }, { where: {id: id} })

    Python
        Staff.objects.filter(id=req_id).update(firstname="Yangi Ism")
        Yoki
        staff = Staff.objects.get(id=req_id)
        staff.firstname = "Yangi Ism"
        staff.save()


# destroy
    Node.js
        Staff.destroy({ where: {id: id} })

    Python
        # 1-usul: Filtr qilib o'chirish
        Staff.objects.filter(id=req_id).delete()

        # 2-usul: Obyektni topib o'chirish
        staff = Staff.objects.get(id=req_id)
        staff.delete()

        
ORM

class Staff(models.Model):
    user = models.ForeignKey(User, on_update=models.CASCADE)
    plan = models.ForeignKey(Plan, on_update=models.CASCADE)
    
    # Django ORM
    result = Staff.objects.select_related('plan').filter(user_id=request.user.id).first()
    all_staff = Staff.objects.select_related('plan', 'katigoriya').filter(user_id=request.user.id)


# Django / Python for
for item in items:
    if item.status == "active":
        print(f"{item.name} faol")
    elif item.status == "pending":
        print("Kutilmoqda")
    else:
        print("Noma'lum")


Node.js,                                Django / Python (pathlib)
__dirname                               BASE_DIR = Path(__file__).resolve().parent.parent
"path.join(__dirname, 'file.txt')",    "Path(__file__).parent / ""file.txt"""
"fs.readFileSync('path', 'utf8')",      Path('path').read_text(encoding='utf-8')
"fs.writeFileSync('path', data)",       Path('path').write_text(data)
fs.existsSync('path'),                  Path('path').exists()
fs.mkdirSync(path)	                    os.mkdir(path)
fs.existsSync(path)	                    os.path.exists(path)
path.join(a, b)	                        os.path.join(a, b)
fs.unlinkSync(oldPath);                 os.remove(old_path)
                                        current_dir = Path(__file__).resolve().parent
                                        file_path = current_dir / "media" / "images" / "logo.png"

JSON
import json
# Node.js: JSON.stringify(obj)
json_data = json.dumps({"status": "ok", "code": 200})

# Node.js: JSON.parse(str)
python_dict = json.loads(json_data)


Node.js (Array),Django                            / Python (List),Izoh
arr.push({id: 1}),"                               arr.append({""id"": 1})",                       Oxiriga element qo'shish
arr.filter(x => x.id > 1),                        [x for x in arr if x['id'] > 1],                List Comprehension orqali filtrlash
arr.find(x => x.id === 1),                        "next((x for x in arr if x['id'] == 1), None)", Shartga mos birinchi elementni olish
arr.map(x => x.name),                             [x['name'] for x in arr],                       Elementlarni o'zgartirib yangi list olish
arr.length,                                       len(arr),                                       Uzunlikni aniqlash


Rasim yuklash
import os
from django.conf import settings
from rest_framework.views import APIView
from rest_framework.response import Response
from .models import Staff

class StaffUpdateView(APIView):
    def put(self, request, id):
        image_file = request.FILES.get('rasm')
        if not image_file:
            return Response({"error": "Rasm yuklanmadi"}, status=400)
        upload_dir = os.path.join(settings.BASE_DIR, "uploads")
        if not os.path.exists(upload_dir):
            os.makedirs(upload_dir, exist_ok=True)
        unique_name = f"{id}.jpg"
        save_path = os.path.join(upload_dir, unique_name)
        with open(save_path, 'wb+') as destination:
            for chunk in image_file.chunks():
                destination.write(chunk)
        Staff.objects.filter(id=id).update(rasm=unique_name)
        return Response({"message": "Muvaffaqiyatli saqlandi"}, status=200)


Boshqa usul
import os
from django.conf import settings

def upload_staff_image(request):
    image = request.FILES.get('rasm')
    staff_id = request.data.get('id') # yoki url dan kelgan id
    
    if image:
        # 1. Uploads papkasini aniqlash
        upload_dir = os.path.join(settings.BASE_DIR, "uploads")
        if not os.path.exists(upload_dir):
            os.makedirs(upload_dir, exist_ok=True)
            
        # 2. Saqlash yo'li
        file_name = f"{staff_id}.jpg"
        save_path = os.path.join(upload_dir, file_name)
        
        # 3. "Buffer"ni faylga yozish
        with open(save_path, 'wb+') as f:
            f.write(image.read()) # Butun bufferni birdatiga yozish
            
        # 4. Bazada faqat nomini saqlash
        Staff.objects.filter(id=staff_id).update(rasm=file_name)



Page limit count olish
from django.db.models import Q

# 1. Shartlarni tayyorlash (Op.or va Op.like)
search_query = Q(firstname__icontains=searchTerm) | \
               Q(lastname__icontains=searchTerm) | \
               Q(middlename__icontains=searchTerm)

# 2. Asosiy so'rov (where, limit, offset)
queryset = Staff.objects.filter(
    search_query,
    user_id=request.user.id,
    status__in=['Aktive', 'Rest'] # Sequelize: status: [...]
)[offset : offset + limit] # Slicing (limit va offset)

# 3. Natijalarni olish (count va rows)
count = Staff.objects.filter(search_query, user_id=request.user.id, status__in=['Aktive', 'Rest']).count()
rows = list(queryset)


Requsetlar
    req.body
        Node.js: req.body.firstname
        Django: request.data.get('firstname') yoki request.data['firstname']

    req.query
        Node.js: req.query.firstname
        Django: request.query_params.get('firstname')

    req.params.id
        async get (req, res) { 
            const id = req.params.id; 
        }

        def get(self, request, id):
            id ...


Auth
settings.py

REST_FRAMEWORK = {
    'DEFAULT_AUTHENTICATION_CLASSES': (
        'rest_framework_simplejwt.authentication.JWTAuthentication',
    ),
    'DEFAULT_PERMISSION_CLASSES': (
        'rest_framework.permissions.IsAuthenticated', # Global himoya (app.use kabi)
    ),
}

# Secret key va vaqt (SECRET_KEY va expiresIn muqobili)
from datetime import timedelta
SIMPLE_JWT = {
    'ACCESS_TOKEN_LIFETIME': timedelta(hours=5),
    'SIGNING_KEY': 'sizning_secret_keyingiz', # Odatda settings.SECRET_KEY ishlatiladi
    'AUTH_HEADER_TYPES': ('Bearer',),
}


# urls.py
from rest_framework_simplejwt.views import TokenObtainPairView
urlpatterns = [
    path('api/login/', TokenObtainPairView.as_view(), name='token_obtain_pair'),
]




Agar sizga aynan Node.js dagi kabi har bir request kirib kelishida ishlaydigan mantiq kerak bo'lsa, middleware.py fayli ochiladi:
Python
# middleware.py
class SimpleMiddleware:
    def __init__(self, get_response):
        self.get_response = get_response

    def __call__(self, request):
        # 1. Request kelganda (Node.js next() dan oldingi qism)
        auth_header = request.headers.get('Authorization')
        response = self.get_response(request) # Bu next() vazifasini bajaradi
        # 2. Response ketayotganda
        return response




from rest_framework.views import APIView
from rest_framework.response import Response
from rest_framework.permissions import IsAuthenticated

class StaffView(APIView):
    # Bu yerda Express'dagi middleware mantiqi ishlaydi
    permission_classes = [IsAuthenticated] 

    def post(self, request):
        # request.data bu Express'dagi req.body bilan bir xil
        data = request.data 
        return Response({"message": "Qabul qilindi"})



from rest_framework.permissions import AllowAny
from rest_framework.views import APIView

# 1. Registration & Login
class RegisterView(APIView):
    permission_classes = [AllowAny] # Ochiq
    def post(self, request): ...

# 2. Bot uchun API
class StaffBotView(APIView):
    permission_classes = [AllowAny] # Ochiq
    def post(self, request): ...

# 3. Event API'lari
class EventView(APIView):
    permission_classes = [IsAuthenticated]  # Yopiq
    def post(self, request): ...


Apilarni boshqarish

    Node.js
        route.get('/getEvent', EventControl.get); // 1
        route.post('/getOneStaffEvent', EventControl.getOneStaffEvent); // 1
        route.post('/getOneStaffEventMonth', EventControl.getOneStaffEventMonth); // 1
        route.get('/expoerExcellEvents', EventControl.expoerExcellEvents); // 1
        route.get('/expoerExcellEventsAll', EventControl.expoerExcellEventsAll); // 1

    Django
    urlpatterns = [
        path('getEvent', EventViewSet.as_view({'get': 'get_events'})),
        path('getOneStaffEvent', EventViewSet.as_view({'post': 'getOneStaffEvent'})),
        path('getOneStaffEventMonth', EventViewSet.as_view({'post': 'getOneStaffEventMonth'})),
        path('expoerExcellEvents', EventViewSet.as_view({'get': 'expoerExcellEvents'})),
        path('expoerExcellEventsAll', EventViewSet.as_view({'get': 'expoerExcellEventsAll'})),
    ]

    controller
        from rest_framework import viewsets
        from rest_framework.decorators import action
        from rest_framework.response import Response

        class EventViewSet(viewsets.ViewSet):
            
            # 1. route.get('/getEvent', ...)
            @action(detail=False, methods=['get'])
            def get_events(self, request):
                return Response({"message": "Barcha eventlar"})

            # 2. route.post('/getOneStaffEvent', ...)
            @action(detail=False, methods=['post'])
            def getOneStaffEvent(self, request):
                staff_id = request.data.get('staff_id')
                return Response({"message": f"{staff_id} uchun eventlar"})

            # 3. route.post('/getOneStaffEventMonth', ...)
            @action(detail=False, methods=['post'])
            def getOneStaffEventMonth(self, request):
                return Response({"message": "Oylik eventlar"})

            # 4. route.get('/expoerExcellEvents', ...)
            @action(detail=False, methods=['get'])
            def expoerExcellEvents(self, request):
                return Response({"message": "Excell yuklanmoqda"})



# middlewarelik varyant

from rest_framework.decorators import action, permission_classes
from rest_framework.permissions import IsAuthenticated
from rest_framework.response import Response

class EventViewSet(viewsets.ViewSet):

    @action(detail=False, methods=['post'])
    @permission_classes([IsAuthenticated])  # Dekoratorda qavslar va ro'yxat bo'lishi shart
    def getOneStaffEvent(self, request):
        # Bu yerga faqat tokeni borlar kira oladi
        staff_id = request.data.get('id')
        return Response({"status": "success", "staff_id": staff_id})

    # 1. detail=False: Hamma xodimlarni analiz qilish (ID shart emas)
    @action(detail=False, methods=['post'])
    def analyze_all(self, request):
        # Mantiq: barcha eventlarni skanerlash
        return Response({"status": "All events analyzed"})

    # 2. detail=True: Faqat bitta aniq xodimning tadbirini tasdiqlash
    @action(detail=True, methods=['post'])
    def approve_event(self, request, pk=None):
        # pk (primary key) bu URL'dagi ID (masalan: 15)
        # Mantiq: ID-si pk bo'lgan eventni topib tasdiqlash
        return Response({"status": f"Event {pk} approved"})